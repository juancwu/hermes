use std::collections::HashMap;
use std::str::Chars;

use crate::parser::token::Token;
use crate::parser::transition_table::{build_transition_table, char_to_input, Input, State};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    chars: Chars<'a>,
    current_char: Option<char>,
    lookahead_char: Option<char>,
    start_index: usize,
    end_index: usize,
    transition_table: HashMap<(State, Input), State>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            chars: input.chars(),
            current_char: None,
            lookahead_char: None,
            start_index: 0,
            end_index: 0,
            transition_table: build_transition_table(),
        };
        // initialize the lexer character position
        lexer.advance();
        // Fill lookahead
        lexer.advance();
        // reset the end index after populating the current and lookahead characters.
        lexer.end_index = 0;
        lexer
    }

    /// Grab the next token that can be identified in the input.
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces_or_newline();

        let mut state = State::Start;
        while let Some(ch) = self.current_char {
            let input_type = char_to_input(ch);
            state = match self.transition_table.get(&(state, input_type)) {
                Some(&new_state) => new_state,
                None => State::Error,
            };
            // println!(
            //     "{}, {:?}, {:?}, {}, {}",
            //     ch, input_type, state, self.start_index, self.end_index
            // );
            // do something here
            match state {
                State::Identifier => {
                    // get the raw string
                    let identifier = self.get_raw_string();
                    // reset the pointer to be at the same place now after grabbing the
                    // identifier
                    self.start_index = self.end_index;
                    return Some(self.match_identifier_to_keyword(identifier));
                }
                State::CurlyBracket => {
                    self.advance();
                    self.start_index = self.end_index;
                    return if ch == '{' {
                        Some(Token::CurlyLeft)
                    } else {
                        Some(Token::CurlyRight)
                    };
                }
                // here we want to check for the reading state instead of the final state because
                // raw values can have escaped characters and it is easier to grab the string with
                // its own loop instead.
                State::ReadingRawValue => {
                    // skip initial double quote
                    self.advance();
                    let mut raw_value = String::new();
                    while let Some(ch) = self.current_char {
                        let input_type = char_to_input(ch);
                        state = match self.transition_table.get(&(state, input_type)) {
                            Some(&new_state) => new_state,
                            None => State::Error,
                        };
                        // println!(
                        //     "{}, {:?}, {:?}, {}, {}",
                        //     ch, input_type, state, self.start_index, self.end_index
                        // );
                        match state {
                            State::ReadingEscapedChar => {
                                // skip the backslash
                                self.advance();
                            }
                            State::ReadingRawValue => {
                                raw_value.push(ch);
                                self.advance();
                            }
                            State::RawValue => {
                                // finished reading the raw value now just stop the loop
                                // this will skip the ending double quote.
                                self.advance();
                                self.start_index = self.end_index;
                                return Some(Token::RawValue(raw_value));
                            }
                            _ => {
                                return Some(Token::Illegal);
                            }
                        }
                    }
                }
                State::Error => {
                    return Some(Token::Illegal);
                }
                State::Comment => {
                    // the transition table should have gotten rid of all the comment characters.
                    self.start_index = self.end_index;
                }
                State::Digit => {
                    self.advance();
                    self.start_index = self.end_index;
                    match ch {
                        '1' => return Some(Token::StateEnabled),
                        _ => return Some(Token::StateDisabled),
                    }
                }
                _ => {
                    self.advance();
                }
            }
        }

        None
    }

    /// Move onto the next character, may be None.
    fn advance(&mut self) {
        // move to end index to later grab the desired input string
        self.end_index += 1;
        self.current_char = self.lookahead_char;
        self.lookahead_char = self.chars.next();
    }

    /// Skip all characters that have the White_Space property. Read Rust documentation for more
    /// information.
    fn skip_whitespaces_or_newline(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() || ch == '\n' {
                self.advance();
            } else {
                break;
            }
        }
        // set the starting pointer to the end now after skipping through whitespaces and newlines.
        self.start_index = self.end_index;
    }

    /// Gets the raw read input that the next_token has read.
    fn get_raw_string(&self) -> String {
        let source = &self.input[self.start_index..self.end_index];
        let raw = String::from(source);
        raw
    }

    fn match_identifier_to_keyword(&self, identifier: String) -> Token {
        match identifier.as_str() {
            "metadata" => Token::MetadataBlock,
            "request" => Token::RequestBlock,
            "collection" => Token::CollectionBlock,
            "headers" => Token::HeadersBlock,
            "queries" => Token::QueriesBlock,
            "environment" => Token::EnvironmentBlock,
            "variables" => Token::VariablesBlock,
            "body" => Token::BodyBlock,
            "folder" => Token::FolderBlock,
            _ => Token::Identifier(identifier),
        }
    }
}
