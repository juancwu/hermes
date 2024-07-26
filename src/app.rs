use std::{collections::HashMap, io, vec};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph, Widget, Wrap},
    Frame,
};

use crate::tui;
use crate::widgets::input::{Input, InputMode};
use crate::{
    api::{Collection, HttpMethod, Request},
    APP_VERSION,
};

/// App is the main application process that will update and render as well as store the
/// application state.
#[derive(Debug)]
pub struct App {
    collection: Collection,
    input_widget: Input,
    open_new_request_popup: bool,
    new_request_step: usize,
    new_request_input_strings_hashmap: HashMap<usize, String>,

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
            input_widget: Input::default(),
            open_new_request_popup: false,
            new_request_step: 0,
            new_request_input_strings_hashmap: new_request_hashmap,
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
        frame.render_widget(self, frame.size());
        match self.input_widget.get_input_mode() {
            InputMode::Normal => {}
            InputMode::Insert => frame.set_cursor(frame.size().x + 1, frame.size().y + 1),
        };
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
                        self.input_widget.change_mode(InputMode::Insert);
                    }
                    KeyCode::Enter if key_event.modifiers == KeyModifiers::CONTROL => {}
                    _ => {}
                }
            }
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press && self.open_new_request_popup =>
            {
                match key_event.code {
                    KeyCode::Char(ch) => {
                        self.input_widget.enter_character(ch);
                    }
                    KeyCode::Backspace => {
                        self.input_widget.delete_character();
                    }
                    KeyCode::Esc => {
                        self.input_widget.reset();
                        self.open_new_request_popup = false;
                    }
                    KeyCode::Enter => {
                        if self.is_end_of_new_request() {
                            let request = Request::new(
                                "test".to_string(),
                                HttpMethod::Get,
                                self.input_widget.get_input_as_string(),
                                None,
                                None,
                                HashMap::new(),
                            );
                            self.collection.add_request(request);
                            self.input_widget.reset();
                            self.open_new_request_popup = false;
                        } else {
                            // if not end, then we move onto the next field
                            self.save_and_move_to_next_new_request_input();
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
        let mut is_end = true;
        for (_step, input_string) in self.new_request_input_strings_hashmap.iter() {
            if input_string.is_empty() {
                is_end = false;
                break;
            }
        }
        is_end
    }

    /// Will save the current input String into the right spot in the input hashmap and move the
    /// step to the next corresponding input.
    ///
    /// IMPORTANT: this method will clear out the current input widget buffer.
    fn save_and_move_to_next_new_request_input(&mut self) {
        let input_string = self.input_widget.get_input_as_string();
        self.new_request_input_strings_hashmap
            .insert(self.new_request_step, input_string);
        self.new_request_step = (self.new_request_step + 1) % 3;
        self.input_widget.reset();
    }

    fn render_new_request_popup(&self, area: Rect, buf: &mut Buffer) {
        let height_per_block = 3;
        let num_of_blocks = 3;
        let popup_height = height_per_block * num_of_blocks;
        // make the popup dimensions
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 2 - popup_height / 2,
            width: area.width / 2,
            height: popup_height,
        };
        Clear.render(popup_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            // constraints are explicitly defined like this to avoid heap allocation
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(popup_area);

        let block_names = vec!["NAME", "METHOD", "URL"];

        // render all the saved input strings to display them, later we will render our current
        // input
        for (step, input_string) in self.new_request_input_strings_hashmap.iter() {
            Paragraph::new(input_string.clone())
                .wrap(Wrap { trim: true })
                .block(
                    Block::bordered()
                        .border_set(border::PLAIN)
                        .title(block_names[*step]),
                )
                .render(chunks[*step], buf);
        }

        // now we will clear the chunk that and override the part where the active input is
        Clear.render(chunks[self.new_request_step], buf);
        Paragraph::new(self.input_widget.get_input_as_string())
            .wrap(Wrap { trim: true })
            .block(
                Block::bordered()
                    .border_set(border::PLAIN)
                    .title(block_names[self.new_request_step]),
            )
            .render(chunks[self.new_request_step], buf);

        // Popup::default()
        //     .content(self.input_widget.get_input_as_string())
        //     .style(match self.input_widget.get_input_mode() {
        //         InputMode::Insert => Style::default().fg(Color::Yellow),
        //         InputMode::Normal => Style::default(),
        //     })
        //     .title("New Request")
        //     .title_style(Style::default().bold())
        //     .border_style(Style::new().light_yellow())
        //     .render(area, buf);
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // split the layout
        // need one line at the bottom for basic instruction hint and app name
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        // render the app name
        Paragraph::new(Text::styled(
            "  <pgUp/pgDn> to scroll, <esc> to cancel, ? for help and q to quit.",
            Style::default().fg(Color::LightBlue),
        ))
        .left_aligned()
        .render(chunks[1], buf);
        Paragraph::new(Text::styled(
            format!("Hermes {} ", APP_VERSION),
            Style::default().fg(Color::LightYellow),
        ))
        .right_aligned()
        .render(chunks[1], buf);

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
            Paragraph::new(
                self.collection
                    .iter()
                    .map(|request| Line::from(request.to_string()))
                    .collect::<Vec<Line>>(),
            )
            .block(block)
            .wrap(Wrap { trim: true })
            .render(side_area, buf);
        } else {
            Paragraph::new("No requests in collection".bold().yellow().to_string())
                .wrap(Wrap { trim: true })
                .block(block)
                .render(side_area, buf);
        };

        Block::bordered().render(request_details_area, buf);

        if self.open_new_request_popup {
            // pass in global area to center the popup.
            self.render_new_request_popup(area, buf)
        }
    }
}
