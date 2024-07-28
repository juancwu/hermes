use std::{collections::HashMap, io, vec};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::tui;
use crate::{
    api::{Collection, HttpMethod, Request},
    instructions,
};

use crate::components;

/// This is the height of a single block/line in the new request popup.
const NEW_REQUEST_HEIGHT_PER_BLOCK: u16 = 3;
/// This is the number of blocks/lines present in the new request popup.
const NEW_REQUEST_NUM_OF_BLOCKS: u16 = 2;
/// This is the total height the new request popup will take. There is an additional 1 to account
/// for the instruction line which doesn't take up 3 spaces.
const NEW_REQUEST_POPUP_HEIGHT: u16 = NEW_REQUEST_HEIGHT_PER_BLOCK * NEW_REQUEST_NUM_OF_BLOCKS + 1;

/// App is the main application process that will update and render as well as store the
/// application state.
#[derive(Debug)]
pub struct App {
    collection: Collection,

    /// Flag controlling
    open_new_request_popup: bool,
    new_request_step: usize,
    new_request_name: components::Input,
    new_request_method: components::List<HttpMethod>,
    new_request_url: components::Input,

    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut new_request_hashmap = HashMap::<usize, String>::new();
        new_request_hashmap.insert(0, String::new());
        new_request_hashmap.insert(1, String::new());
        new_request_hashmap.insert(2, String::new());
        App {
            collection: Collection::default(),
            open_new_request_popup: false,
            new_request_step: 0,
            new_request_name: components::Input::new().title("Name"),
            new_request_method: components::List::default()
                .items(vec![
                    HttpMethod::Get,
                    HttpMethod::Post,
                    HttpMethod::Patch,
                    HttpMethod::Put,
                    HttpMethod::Delete,
                    HttpMethod::Option,
                ])
                .title("Method"),
            new_request_url: components::Input::new().title("Url"),
            exit: false,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.view(frame))?;
            self.update()?;
        }
        Ok(())
    }

    /// Render the view for the model
    fn view(&self, frame: &mut Frame) {
        let area = frame.size();
        // split the layout
        // need one line at the bottom for basic instruction hint and app name
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        // render the app name
        frame.render_widget(
            instructions!("<pgUp/pgDn> to scroll, <esc> to cancel, ? for help and q to quit.")
                .left_aligned(),
            chunks[1],
        );
        // .render(chunks[1], buf);
        let app_name = Paragraph::new(Text::styled(
            format!("Hermes {} ", "0.1.0"),
            Style::default().fg(Color::LightYellow),
        ))
        .right_aligned();
        frame.render_widget(app_name, chunks[1]);

        // main area layout
        // split into two main columns
        // column 1: contains a list of the collections that have been read into memory along with
        // all the requests in the collection
        // column 2: shows the details of a selected request in the collection. Details explained
        // in comments down below when column 2 is being rendered.
        let main_area_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Length(1),
                Constraint::Percentage(80),
            ])
            .split(chunks[0]);

        // render the side area with the requests in the current collection.
        let side_area = main_area_chunks[0];
        self.render_collection_requests(side_area, frame);

        // render the main area with the request details
        let request_details_area = main_area_chunks[2];
        frame.render_widget(Block::bordered(), request_details_area);

        if self.open_new_request_popup {
            // pass in global area to center the popup.
            self.render_new_request_popup(frame);
        }
    }

    /// Update the state of the model
    fn update(&mut self) -> io::Result<()> {
        match event::read()? {
            // Make sure to check if key event is 'press' since crossterm also emits 'release' and
            // 'repeat' events.
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press && !self.open_new_request_popup =>
            {
                match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('a') => {
                        self.open_new_request_popup = true;
                        self.new_request_name.enable_insert_mode();
                    }
                    KeyCode::Enter if key_event.modifiers == KeyModifiers::CONTROL => {}
                    _ => {}
                }
            }
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press && self.open_new_request_popup =>
            {
                match key_event.code {
                    KeyCode::Char(ch) => match self.new_request_step {
                        0 => self.new_request_name.enter_character(ch),
                        1 => match ch {
                            'j' => self.new_request_method.next(),
                            'k' => self.new_request_method.prev(),
                            _ => {}
                        },
                        2 => self.new_request_url.enter_character(ch),
                        _ => {}
                    },
                    KeyCode::Backspace => match self.new_request_step {
                        0 => self.new_request_name.delete_character(),
                        2 => self.new_request_url.delete_character(),
                        _ => {}
                    },
                    KeyCode::Esc => {
                        self.new_request_name.reset();
                        self.new_request_url.reset();
                        self.open_new_request_popup = false;
                        self.new_request_step = 0;
                    }
                    KeyCode::Tab => {
                        self.move_to_next_new_request_step();
                    }
                    KeyCode::Enter => {
                        if self.is_end_of_new_request() {
                            let request = Request::new(
                                self.new_request_name.get_string(),
                                match self.new_request_method.get_selected() {
                                    Some(method) => method,
                                    None => HttpMethod::Get,
                                },
                                self.new_request_url.get_string(),
                                None,
                                None,
                                HashMap::new(),
                            );
                            self.collection.add_request(request);
                            self.open_new_request_popup = false;
                            self.new_request_name.reset();
                            self.new_request_url.reset();
                            self.new_request_method.reset();
                            self.new_request_step = 0;
                        } else {
                            // if not end, then we move onto the next field
                            self.move_to_next_new_request_step();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }

    /// Checks whether all the fields for a new request has been filled.
    /// For now we are just checking of empty fields but should also check/validate the inputs?
    fn is_end_of_new_request(&self) -> bool {
        !self.new_request_name.is_empty() && !self.new_request_url.is_empty()
    }

    /// Will save the current input String into the right spot in the input hashmap and move the
    /// step to the next corresponding input.
    ///
    /// IMPORTANT: this method will clear out the current input widget buffer.
    fn move_to_next_new_request_step(&mut self) {
        match self.new_request_step {
            0 => {
                self.new_request_name.enable_normal_mode();
            }
            1 => {
                self.new_request_url.enable_insert_mode();
            }
            2 => {
                self.new_request_name.enable_insert_mode();
                self.new_request_url.enable_normal_mode();
            }
            _ => {}
        };
        self.new_request_step = (self.new_request_step + 1) % 3;
        self.new_request_method
            .set_focus(self.new_request_step == 1);
    }

    /// Renders the side area where all the requests from the currently opened Collection. This
    /// method takes a Rect to actually know where the sidebar is instead of deciding by itself
    /// where it should. Since this is more like a "component", it should not care about where it
    /// is going to be used, just how.
    fn render_collection_requests(&self, area: Rect, frame: &mut Frame) {
        let block = Block::bordered().title(self.collection.name());
        if self.collection.is_empty() {
            frame.render_widget(
                Paragraph::new(
                    Text::from("No requests in collection").style(Style::new().fg(Color::Yellow)),
                )
                .block(block),
                area,
            )
        } else {
            // divide the area into possible blocks that can be displayed
            // the total blocks can be calculated using area.height / 4 where 2 lines is taken by
            // a block's border and 2 more lines for request name, method and url
            let num_of_blocks = area.height / 4;
            let mut chunk_constraints = Vec::<Constraint>::new();
            for _ in 0..num_of_blocks {
                chunk_constraints.push(Constraint::Length(4));
            }
            let chunks = Layout::new(Direction::Vertical, chunk_constraints).split(area);
            for _ in 0..num_of_blocks {}
            for (index, request) in self
                .collection
                .iter()
                .take(num_of_blocks as usize)
                .enumerate()
            {
                let method = request.get_method();
                let name = request.get_name();
                let url = request.get_url();
                let first_line = Line::from(name);
                let second_line = Line::from(vec![
                    Span::from(method.to_str()).style(Style::new().fg(method.color())),
                    " ".into(),
                    Span::from(url),
                ]);
                let paragraph = Paragraph::new(vec![first_line, second_line]).block(
                    Block::bordered().style(Style::default().fg(if index == 0 {
                        Color::LightYellow
                    } else {
                        Color::default()
                    })),
                );
                frame.render_widget(paragraph, chunks[index]);
            }
        }
    }

    fn render_new_request_popup(&self, frame: &mut Frame) {
        let area = frame.size();
        // make the popup dimensions
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 2 - NEW_REQUEST_POPUP_HEIGHT / 2,
            width: area.width / 2,
            height: NEW_REQUEST_POPUP_HEIGHT,
        };
        // clear area, avoid things underneath leaked into the popup
        frame.render_widget(Clear, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            // constraints are explicitly defined like this to avoid heap allocation
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(popup_area);

        // instructions for method list
        frame.render_widget(
            instructions!(if self.new_request_step == 1 {
                "Use j/k to change method."
            } else {
                "Start typing."
            })
            .left_aligned(),
            chunks[2],
        );

        // instructions to exit the popup
        frame.render_widget(instructions!("<esc> to cancel.").right_aligned(), chunks[2]);

        // separate the area for the method and url
        let url_chunks = layout::Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                layout::Constraint::Percentage(20),
                layout::Constraint::Percentage(80),
            ])
            .split(chunks[1]);

        // render all inputs
        frame.render_widget(self.new_request_name.clone(), chunks[0]);
        frame.render_widget(self.new_request_method.clone(), url_chunks[0]);
        frame.render_widget(self.new_request_url.clone(), url_chunks[1]);

        // set cursor
        match self.new_request_step {
            0 => frame.set_cursor(
                chunks[0].x + 1 + self.new_request_name.get_cursor_index_u16(),
                chunks[0].y + 1,
            ),
            2 => frame.set_cursor(
                url_chunks[1].x + 1 + self.new_request_url.get_cursor_index_u16(),
                url_chunks[1].y + 1,
            ),
            _ => {}
        }
    }
}
