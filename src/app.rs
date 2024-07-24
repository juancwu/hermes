use std::io;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

use crate::tui;

#[derive(Debug, Default)]
pub struct App {
    counter: i32,
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
                    KeyCode::Up => self.counter += 1,
                    KeyCode::Down => self.counter -= 1,
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

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
