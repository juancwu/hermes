use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
    // Definitive states, means that something has finished reading
    Start,
    Identifier,
    Delimeter,
    RawValue,
    Comment,
    Error,

    // Transitional states, means that it should continue going through the input and transition
    // table.
    ReadingIdentifier,
    ReadingRawValue,
    ReadingComment,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Input {
    Character,
    DoubleQuote,
    Delimeter,
    Whitespace,
    NewLine,
    Comment,
}

/// Match the given character with an Input type to use with a transition table.
pub fn char_to_input(ch: char) -> Input {
    match ch {
        ' ' | '\t' => Input::Whitespace,
        '\n' => Input::NewLine,
        '"' => Input::DoubleQuote,
        '{' | '}' => Input::Delimeter,
        '#' => Input::Comment,
        _ => Input::Character,
    }
}

/// Builds a transition table to use to create lexemes.
pub fn build_transition_table() -> HashMap<(State, Input), State> {
    let mut table: HashMap<(State, Input), State> = HashMap::new();

    // ignore whitespaces
    table.insert((State::Start, Input::Whitespace), State::Start);
    table.insert((State::Start, Input::NewLine), State::Start);
    table.insert((State::Start, Input::Comment), State::ReadingComment);
    table.insert((State::Start, Input::Character), State::ReadingIdentifier);
    table.insert((State::Start, Input::Delimeter), State::Delimeter);

    // Reading identifier
    table.insert(
        (State::ReadingIdentifier, Input::Character),
        State::ReadingIdentifier,
    );
    table.insert(
        (State::ReadingIdentifier, Input::DoubleQuote),
        State::Identifier,
    );
    table.insert(
        (State::ReadingIdentifier, Input::Whitespace),
        State::Identifier,
    );
    table.insert(
        (State::ReadingIdentifier, Input::Delimeter),
        State::Identifier,
    );
    table.insert(
        (State::ReadingIdentifier, Input::Comment),
        State::Identifier,
    );

    // Reading comment
    table.insert(
        (State::ReadingComment, Input::Character),
        State::ReadingComment,
    );
    table.insert(
        (State::ReadingIdentifier, Input::DoubleQuote),
        State::ReadingComment,
    );
    table.insert(
        (State::ReadingIdentifier, Input::Delimeter),
        State::ReadingComment,
    );
    table.insert(
        (State::ReadingIdentifier, Input::Comment),
        State::ReadingComment,
    );
    table.insert(
        (State::ReadingIdentifier, Input::Whitespace),
        State::ReadingComment,
    );
    table.insert((State::ReadingIdentifier, Input::NewLine), State::Comment);

    // Reading raw value
    table.insert(
        (State::ReadingRawValue, Input::NewLine),
        State::ReadingRawValue,
    );
    table.insert(
        (State::ReadingRawValue, Input::Whitespace),
        State::ReadingRawValue,
    );
    table.insert(
        (State::ReadingRawValue, Input::Character),
        State::ReadingRawValue,
    );
    table.insert(
        (State::ReadingRawValue, Input::Comment),
        State::ReadingRawValue,
    );
    table.insert(
        (State::ReadingRawValue, Input::Delimeter),
        State::ReadingRawValue,
    );
    table.insert(
        (State::ReadingIdentifier, Input::DoubleQuote),
        State::RawValue,
    );

    table
}

/// Check if given state is a definitive state which indicates a stop to read current string.
pub fn is_definitive_state(state: State) -> bool {
    match state {
        State::ReadingIdentifier | State::ReadingComment | State::ReadingRawValue => false,
        _ => true,
    }
}
