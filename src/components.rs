//! This file is named components.rs to not cause conflicts with ratatui::widgets for suggestions.

use derive_setters::*;
use ratatui::widgets::{Paragraph, Widget};

#[macro_export]
macro_rules! instructions {
    ($text:expr) => {
        Paragraph::new($text).style(Style::new().fg(Color::LightBlue))
    };
}

/// Different input modes for the Input component. Nothing gets registered outside of Insert mode.
#[derive(Debug, Default, Clone, Copy)]
pub enum InputMode {
    #[default]
    Normal,
    Insert,
}

/// Input represents an input widget that only cares about getting inputs but not how it looks.
#[derive(Debug, Default, Clone, Setters)]
pub struct Input {
    /// The current input
    #[setters(skip)]
    input: String,
    /// The current input mode
    #[setters(skip)]
    input_mode: InputMode,
    /// The current position of the cursor
    #[setters(skip)]
    cursor_index: usize,
    /// Input title
    #[setters(into)]
    title: String,
    /// The style for the borders
    border_style: ratatui::style::Style,
    /// The style for the borders when input mode is insert. Default style is blue borders.
    insert_mode_border_style: ratatui::style::Style,
    /// The styles for the input text
    style: ratatui::style::Style,
    /// The styles for the input text when input mode is insert. Default style is blue text.
    insert_mode_style: ratatui::style::Style,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::default(),
            cursor_index: 0,
            title: String::new(),
            border_style: ratatui::style::Style::default(),
            insert_mode_border_style: ratatui::style::Style::new()
                .fg(ratatui::style::Color::Yellow),
            style: ratatui::style::Style::default(),
            insert_mode_style: ratatui::style::Style::new().fg(ratatui::style::Color::Yellow),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }

    pub fn enter_character(&mut self, character: char) {
        match self.input_mode {
            InputMode::Insert => {
                self.input.insert(self.cursor_index, character);
                self.move_cursor_right();
            }
            // ignore all other modes
            _ => {}
        };
    }

    pub fn delete_character(&mut self) {
        if self.input.chars().count() > 0 {
            // the cursor index is always one ahead of the input
            let current_index = self.cursor_index;
            let left_to_current_index = current_index - 1;
            let before_delete_chars = self.input.chars().take(left_to_current_index);
            let after_delete_chars = self.input.chars().skip(current_index);
            self.input = before_delete_chars.chain(after_delete_chars).collect();
            self.move_cursor_left();
        }
    }

    pub fn enable_normal_mode(&mut self) {
        self.set_input_mode(InputMode::Normal);
    }

    pub fn enable_insert_mode(&mut self) {
        self.set_input_mode(InputMode::Insert);
    }

    /// Gets the entire input and resets the states of the input widget.
    pub fn get_string(&self) -> String {
        self.input.clone()
    }

    pub fn get_cursor_index_u16(&self) -> u16 {
        match u16::try_from(self.cursor_index) {
            Ok(v) => v,
            Err(_) => 0,
        }
    }

    /// Reset the states of the input widget
    pub fn reset(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input.clear();
        self.cursor_index = 0;
    }

    /// Moves the cursors by the right.
    fn move_cursor_right(&mut self) {
        let new_cursor_index = self.cursor_index.saturating_add(1);
        self.cursor_index = new_cursor_index.clamp(0, self.input.chars().count());
    }

    /// Moves the cursor by the left.
    fn move_cursor_left(&mut self) {
        let new_cursor_index = self.cursor_index.saturating_sub(1);
        self.cursor_index = new_cursor_index.clamp(0, self.input.chars().count());
    }

    fn set_input_mode(&mut self, mode: InputMode) {
        self.input_mode = mode;
    }
}

impl Widget for Input {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        ratatui::widgets::Paragraph::new(self.input)
            .block(
                ratatui::widgets::Block::bordered()
                    .style(match self.input_mode {
                        InputMode::Normal => self.border_style,
                        InputMode::Insert => self.insert_mode_border_style,
                    })
                    .title(self.title),
            )
            .style(match self.input_mode {
                InputMode::Normal => self.style,
                InputMode::Insert => self.insert_mode_style,
            })
            .render(area, buf);
    }
}

#[derive(Debug, Clone, Setters)]
pub struct List<T> {
    /// The items available in the List. The List defaults to have "None" item if no items were
    #[setters(into)]
    items: Vec<T>,
    /// The currently selected item's index.
    #[setters(skip)]
    selected_index: usize,
    /// The title of the List.
    #[setters(into)]
    title: String,
    /// The style for the borders
    border_style: ratatui::style::Style,
    /// The style for the borders when list is focused. Default style is yellow borders.
    focus_border_style: ratatui::style::Style,
    /// The styles for the text
    style: ratatui::style::Style,
    /// The styles for the text when list is focused. Default style is yellow text.
    focus_style: ratatui::style::Style,
    /// Flag that determines if list is focused or not.
    #[setters(skip)]
    is_focused: bool,
}

impl<T: Clone> List<T> {
    /// Move to the next item in List.
    pub fn next(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.items.len();
    }

    // Move to the previous item in List.
    pub fn prev(&mut self) {
        self.selected_index = if self.selected_index == 0 {
            self.items.len() - 1
        } else {
            (self.selected_index - 1) % self.items.len()
        };
    }

    /// Get the value of the selected item in the List.
    pub fn get_selected(&self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items[self.selected_index].clone())
        }
    }
}

impl<T: ToString + Clone> Widget for List<T> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.items[self.selected_index].to_string())
            .block(
                ratatui::widgets::Block::bordered()
                    .title(self.title)
                    .border_style(if self.is_focused {
                        self.focus_border_style
                    } else {
                        self.border_style
                    }),
            )
            .style(if self.is_focused {
                self.focus_style
            } else {
                self.style
            })
            .render(area, buf);
    }
}

impl<T: Default + Clone> Default for List<T> {
    fn default() -> Self {
        Self {
            items: vec![T::default()],
            selected_index: 0,
            title: String::from(""),
            border_style: ratatui::style::Style::default(),
            focus_border_style: ratatui::style::Style::new().fg(ratatui::style::Color::Yellow),
            style: ratatui::style::Style::default(),
            focus_style: ratatui::style::Style::new().fg(ratatui::style::Color::Yellow),
            is_focused: false,
        }
    }
}
