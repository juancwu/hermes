use std::{char, collections::HashMap, io};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

use crate::api::{Collection, HttpMethod, Request};
use crate::tui;
use crate::widgets::input::{Input, InputMode};

/// App is the main application process that will update and render as well as store the
/// application state.
#[derive(Debug, Default)]
pub struct App {
    collection: Collection,
    input_widget: Input,
    open_input_window: bool,
    url: String,
    exit: bool,
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
                if key_event.kind == KeyEventKind::Press && !self.open_input_window =>
            {
                match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('a') => {
                        self.open_input_window = true;
                        self.input_widget.change_mode(InputMode::Insert);
                    }
                    KeyCode::Enter if key_event.modifiers == KeyModifiers::CONTROL => {}
                    _ => {}
                }
            }
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press && self.open_input_window =>
            {
                match key_event.code {
                    KeyCode::Char(ch) => {
                        self.input_widget.enter_character(ch);
                    }
                    KeyCode::Backspace => {
                        self.input_widget.delete_character();
                    }
                    KeyCode::Enter => {
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
                        self.open_input_window = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Hermes");
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::PLAIN);

        let route_text = if !self.collection.is_empty() {
            Text::from(
                self.collection
                    .iter()
                    .map(|req| req.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        } else {
            Text::from("No requests in collection".bold().yellow().to_string())
        };

        if self.open_input_window {
            let block = Block::bordered().border_set(border::THICK).title("URL");
            Paragraph::new(self.input_widget.get_input_as_string())
                .style(match self.input_widget.get_input_mode() {
                    InputMode::Insert => Style::default().fg(Color::Yellow),
                    InputMode::Normal => Style::default(),
                })
                .centered()
                .block(block)
                .render(area, buf);
        } else {
            Paragraph::new(route_text)
                .centered()
                .block(block)
                .render(area, buf);
        }
    }
}
