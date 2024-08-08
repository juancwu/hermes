use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
    // Definitive states, means that something has finished reading
    Start,
    Identifier,
    CurlyBracket,
    RawValue,
    Comment,
    Digit,
    BlockName,
    Error,

    // Transitional states, means that it should continue going through the input and transition
    // table.
    ReadingIdentifier,
    ReadingRawValue,
    ReadingComment,
    ReadingEscapedChar,
    StartBlockName,
    ReadingBlockName,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Input {
    Character,
    DoubleQuote,
    CurlyBracket,
    Whitespace,
    NewLine,
    Comment,
    BackSlash,
    Digit,
    Colon,
}

/// Match the given character with an Input type to use with a transition table.
pub fn char_to_input(ch: char) -> Input {
    match ch {
        ' ' | '\t' => Input::Whitespace,
        '\n' => Input::NewLine,
        '"' => Input::DoubleQuote,
        '{' | '}' => Input::CurlyBracket,
        '#' => Input::Comment,
        '\\' => Input::BackSlash,
        '0'..='9' => Input::Digit,
        ':' => Input::Colon,
        _ => Input::Character,
    }
}

/// Builds a transition table to use to create lexemes.
pub fn build_transition_table() -> HashMap<(State, Input), State> {
    let mut table: HashMap<(State, Input), State> = HashMap::new();

    let variations = vec![
        ((State::Start, Input::Character), State::ReadingIdentifier),
        ((State::Start, Input::DoubleQuote), State::ReadingRawValue),
        ((State::Start, Input::CurlyBracket), State::CurlyBracket),
        ((State::Start, Input::BackSlash), State::Error),
        ((State::Start, Input::Whitespace), State::Start),
        ((State::Start, Input::NewLine), State::Start),
        ((State::Start, Input::Comment), State::ReadingComment),
        ((State::Start, Input::Digit), State::Digit),
        ((State::Start, Input::Colon), State::StartBlockName),
        // Reading identifier states
        (
            (State::ReadingIdentifier, Input::Character),
            State::ReadingIdentifier,
        ),
        (
            (State::ReadingIdentifier, Input::DoubleQuote),
            State::Identifier,
        ),
        (
            (State::ReadingIdentifier, Input::CurlyBracket),
            State::Identifier,
        ),
        ((State::ReadingIdentifier, Input::BackSlash), State::Error),
        (
            (State::ReadingIdentifier, Input::Whitespace),
            State::Identifier,
        ),
        (
            (State::ReadingIdentifier, Input::NewLine),
            State::Identifier,
        ),
        (
            (State::ReadingIdentifier, Input::Comment),
            State::Identifier,
        ),
        (
            (State::ReadingIdentifier, Input::Digit),
            State::ReadingIdentifier,
        ),
        // Reading comment states
        (
            (State::ReadingComment, Input::Character),
            State::ReadingComment,
        ),
        (
            (State::ReadingComment, Input::DoubleQuote),
            State::ReadingComment,
        ),
        (
            (State::ReadingComment, Input::CurlyBracket),
            State::ReadingComment,
        ),
        (
            (State::ReadingComment, Input::BackSlash),
            State::ReadingComment,
        ),
        (
            (State::ReadingComment, Input::Whitespace),
            State::ReadingComment,
        ),
        ((State::ReadingComment, Input::NewLine), State::Comment),
        (
            (State::ReadingComment, Input::Comment),
            State::ReadingComment,
        ),
        ((State::ReadingComment, Input::Digit), State::ReadingComment),
        // Reading raw values
        (
            (State::ReadingRawValue, Input::Character),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingRawValue, Input::DoubleQuote),
            State::RawValue,
        ),
        (
            (State::ReadingRawValue, Input::CurlyBracket),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingRawValue, Input::BackSlash),
            State::ReadingEscapedChar,
        ),
        (
            (State::ReadingRawValue, Input::Whitespace),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingRawValue, Input::NewLine),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingRawValue, Input::Comment),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingRawValue, Input::Digit),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingRawValue, Input::Colon),
            State::ReadingRawValue,
        ),
        // Reading escaped characters
        (
            (State::ReadingEscapedChar, Input::Character),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingEscapedChar, Input::DoubleQuote),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingEscapedChar, Input::CurlyBracket),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingEscapedChar, Input::BackSlash),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingEscapedChar, Input::Comment),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingEscapedChar, Input::Digit),
            State::ReadingRawValue,
        ),
        (
            (State::ReadingEscapedChar, Input::Colon),
            State::ReadingRawValue,
        ),
        // Start block name read, maybe it is a block name
        (
            (State::StartBlockName, Input::Colon),
            State::ReadingBlockName,
        ),
        // Reading block name
        (
            (State::ReadingBlockName, Input::Character),
            State::ReadingBlockName,
        ),
        ((State::ReadingBlockName, Input::NewLine), State::BlockName),
        (
            (State::ReadingBlockName, Input::Whitespace),
            State::BlockName,
        ),
    ];

    insert_start_states(&mut table);
    table
}

fn insert_start_states(table: &mut HashMap<(State, Input), State>) {
    table.insert((State::Start, Input::Character), State::ReadingIdentifier);
    table.insert((State::Start, Input::DoubleQuote), State::ReadingRawValue);
    table.insert((State::Start, Input::CurlyBracket), State::CurlyBracket);
    table.insert((State::Start, Input::BackSlash), State::Error);
    table.insert((State::Start, Input::Whitespace), State::Start);
    table.insert((State::Start, Input::NewLine), State::Start);
    table.insert((State::Start, Input::Comment), State::ReadingComment);
    table.insert((State::Start, Input::Digit), State::Digit);
    table.insert((State::Start, Input::Colon), State::StartBlockName);
}
