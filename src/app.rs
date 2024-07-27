use std::{collections::HashMap, io};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{self, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
    Frame,
};

use crate::tui;
use crate::widgets::input::Input;
use crate::{
    api::{Collection, HttpMethod, Request},
    APP_VERSION,
};

/// App is the main application process that will update and render as well as store the
/// application state.
#[derive(Debug)]
pub struct App {
    collection: Collection,
    open_new_request_popup: bool,
    new_request_step: usize,
    new_request_name: Input,
    new_request_method: HttpMethod,
    new_request_url: Input,

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
            new_request_name: Input::default(),
            new_request_method: HttpMethod::Get,
            new_request_url: Input::default(),
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
        let global_instructions = Paragraph::new(Text::styled(
            "  <pgUp/pgDn> to scroll, <esc> to cancel, ? for help and q to quit.",
            Style::default().fg(Color::LightBlue),
        ))
        .left_aligned();
        frame.render_widget(global_instructions, chunks[1]);
        // .render(chunks[1], buf);
        let app_name = Paragraph::new(Text::styled(
            format!("Hermes {} ", APP_VERSION),
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
        let side_area = main_area_chunks[0];
        let request_details_area = main_area_chunks[2];

        let block = Block::bordered().title(self.collection.name());
        if !self.collection.is_empty() {
            let requests = Paragraph::new(
                self.collection
                    .iter()
                    .map(|request| Line::from(request.to_string()))
                    .collect::<Vec<Line>>(),
            )
            .block(block)
            .wrap(Wrap { trim: true });
            frame.render_widget(requests, side_area);
        } else {
            let no_requests =
                Paragraph::new("No requests in collection".bold().yellow().to_string())
                    .wrap(Wrap { trim: true })
                    .block(block);
            frame.render_widget(no_requests, side_area);
        };

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
                            'j' => self.new_request_method = self.new_request_method.next(),
                            'k' => self.new_request_method = self.new_request_method.prev(),
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
                                HttpMethod::Get,
                                self.new_request_url.get_string(),
                                None,
                                None,
                                HashMap::new(),
                            );
                            self.collection.add_request(request);
                            self.open_new_request_popup = false;
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
        self.new_request_step = (self.new_request_step + 1) % 3;
    }

    fn render_new_request_popup(&self, frame: &mut Frame) {
        let area = frame.size();
        let height_per_block = 3;
        let num_of_blocks = 2;
        // account the last line for instructions
        let popup_height = height_per_block * num_of_blocks + 1;
        // make the popup dimensions
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 2 - popup_height / 2,
            width: area.width / 2,
            height: popup_height,
        };
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

        let instructions = if self.new_request_step == 1 {
            Paragraph::new("Use j/k to change method.")
                .style(Style::new().fg(Color::LightBlue))
                .left_aligned()
        } else {
            Paragraph::new("Start typing.")
                .style(Style::new().fg(Color::LightBlue))
                .left_aligned()
        };
        frame.render_widget(instructions, chunks[2]);

        let instructions = Paragraph::new("<esc> to cancel.")
            .style(Style::new().fg(Color::LightBlue))
            .right_aligned();
        frame.render_widget(instructions, chunks[2]);

        let request_name = Paragraph::new(self.new_request_name.get_string()).block(
            Block::bordered()
                .title("Name")
                .style(if self.new_request_step == 0 {
                    Style::new().fg(Color::Yellow)
                } else {
                    Style::default()
                }),
        );
        frame.render_widget(request_name, chunks[0]);
        // divide second line into two for method and url
        let url_chunks = layout::Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                layout::Constraint::Percentage(20),
                layout::Constraint::Percentage(80),
            ])
            .split(chunks[1]);
        let request_method = Paragraph::new(self.new_request_method.to_str()).block(
            Block::bordered()
                .title("Method")
                .style(if self.new_request_step == 1 {
                    Style::new().fg(Color::Yellow)
                } else {
                    Style::default()
                }),
        );
        frame.render_widget(request_method, url_chunks[0]);
        let request_url = Paragraph::new(self.new_request_url.get_string()).block(
            Block::bordered()
                .title("Url")
                .style(if self.new_request_step == 2 {
                    Style::new().fg(Color::Yellow)
                } else {
                    Style::default()
                }),
        );
        frame.render_widget(request_url, url_chunks[1]);

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
