use crate::parser::token::Token;

#[derive(Debug, Clone)]
pub struct Lexer {
    current_character: char,
    current_string: String,
    current_position: usize,
    leading_position: usize,
    input_string: String,
    available_bytes: i64,
}

impl Lexer {
    pub fn new(input_string: String) -> Self {
        Self {
            current_character: '\0',
            current_string: String::new(),
            current_position: 0,
            leading_position: 0,
            input_string,
            available_bytes: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.is_eof() {
            return Token::EOF;
        }

        self.skip_whitespace();

        let mut token: Token = Token::EOF;

        while !self.is_eof() {
            match self.current_character {
                '"' => {
                    // skip initial double quotes
                    let mut escape = false;
                    while !self.is_eof() && self.read_character().is_some() {
                        if escape {
                            self.current_string.push(self.current_character);
                            escape = false;
                            continue;
                        }
                        if self.current_character == '\\' {
                            escape = true;
                            continue;
                        }
                        if self.current_character == '"' {
                            break;
                        }
                        self.current_string.push(self.current_character);
                    }
                    token = Token::RawValue(self.current_string.clone())
                }
                '{' => {
                    token = Token::CurlyLeft;
                }
                '}' => {
                    token = Token::CurlyRight;
                }
                ':' => {
                    token = Token::DoubleColon;
                }
                _ => {
                    token = Token::Illegal;
                }
            }
        }

        self.current_string.clear();

        token
    }

    /// Reads a raw string from the input source.
    fn read_raw_string(&mut self, include_linefeed: bool) {}

    fn read_character(&mut self) -> Option<char> {
        if self.is_eof() {
            return None;
        }

        self.current_character = match self.input_string.chars().nth(self.leading_position) {
            Some(ch) => ch,
            None => '\0',
        };

        self.current_position = self.leading_position;
        self.leading_position += 1;

        Some(self.current_character)
    }

    fn peek_character(&self) -> Option<char> {
        if self.is_eof() {
            return None;
        }
        self.input_string.chars().nth(self.leading_position)
    }

    fn is_eof(&self) -> bool {
        self.leading_position >= self.input_string.len()
    }

    fn skip_whitespace(&mut self) {
        while self.current_character == ' ' {
            self.read_character();
        }
    }
}
