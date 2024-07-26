use std::{collections::HashMap, io};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Padding, Paragraph, Widget, Wrap},
    Frame,
};

use crate::tui;
use crate::widgets::{
    input::{Input, InputMode},
    popup::Popup,
};
use crate::{
    api::{Collection, HttpMethod, Request},
    APP_VERSION,
};

/// App is the main application process that will update and render as well as store the
/// application state.
#[derive(Debug, Default)]
pub struct App {
    collection: Collection,
    input_widget: Input,
    open_new_request_popup: bool,
    new_request_step: u8,

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
                        self.open_new_request_popup = false;
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
        let request_details_are = main_area_chunks[2];

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

        Block::bordered().render(request_details_are, buf);

        if self.open_new_request_popup {
            // make the popup dimensions
            let popup_area = Rect {
                x: chunks[0].width / 4,
                y: chunks[0].height / 4,
                width: chunks[0].width / 2,
                height: chunks[0].height / 2,
            };
            Popup::default()
                .content(self.input_widget.get_input_as_string())
                .style(match self.input_widget.get_input_mode() {
                    InputMode::Insert => Style::default().fg(Color::Yellow),
                    InputMode::Normal => Style::default(),
                })
                .title("New Request")
                .title_style(Style::default().bold())
                .border_style(Style::new().light_yellow())
                .render(popup_area, buf);
        }
    }
}
