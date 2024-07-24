use std::{collections::HashMap, io};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

use crate::api::{self, Collection, HttpMethod};
use crate::tui;

/// App is the main application process that will update and render as well as store the
/// application state.
#[derive(Debug, Default)]
pub struct App {
    collection: Collection,
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
    }

    /// Update the state of the model
    fn update(&mut self) -> io::Result<()> {
        match event::read()? {
            // Make sure to check if key event is 'press' since crossterm also emits 'release' and
            // 'repeat' events.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('a') => {
                        let request = api::Request::new(
                            "Healthcheck".to_string(),
                            HttpMethod::Get,
                            "https://konbini.juancwu.dev".to_string(),
                            None,
                            None,
                            HashMap::new(),
                        );
                        self.collection.add_request(request);
                    }
                    KeyCode::Enter if key_event.modifiers == KeyModifiers::CONTROL => {}
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
            .border_set(border::THICK);

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

        Paragraph::new(route_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
