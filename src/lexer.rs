use std::collections::HashMap;
use std::str::Chars;

use crate::transition_table::{build_transition_table, char_to_input, Input, State};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// Represents illegal characters that shouldn't be in the .hermes syntax
    Illegal,
    /// Defines a named block.
    BlockIdentifier(String),
    RequestBlock,
    BodyBlock,
    HeadersBlock,
    QueriesBlock,
    EnvironmentBlock,
    VariablesBlock,
    CollectionBlock,
    FolderBlock,
    /// Typical identifier in any language. This will mostly just be
    /// block names that are used to reference to defined blocks or for reserved keywords.
    ///
    /// Keep in mind that identifier keywords only appear at the beginning of any line in a block.
    ///
    /// Available identifier keywords:
    /// type - type of hermes file, usually defined in a metadata block
    /// name - the type of a collection, request, or folder
    /// text - text type of multipart form data field
    /// file - file type of multipart form data field
    /// environment - use an enviroment
    /// add - add a single request
    /// include - include all requests from a given path
    Identifier(String),
    /// Refers to any raw value read from a hermes file. For example, the JSON body string would be
    /// a raw value, as well as the value of a query parameter.
    RawValue(String),
    /// Some blocks such as headers, queries, form-urlencoded, and mutipart-form can have enabled
    /// fields which are included in the request.
    StateEnabled,
    /// Some blocks such as headers, queries, form-urlencoded, and mutipart-form can have disabled
    /// fields which are included in the request.
    StateDisabled,
    CurlyLeft,
    CurlyRight,
}

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
