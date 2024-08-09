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

    // Transitional states, means that it should continue going through the input and transition
    // table.
    ReadingIdentifier,
    ReadingRawValue,
    ReadingComment,
    ReadingEscapedChar,
    StartBlockName,
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

    insert_start_states(&mut table);
    insert_identifier_states(&mut table);
    insert_comment_states(&mut table);
    insert_escaped_char_states(&mut table);
    insert_raw_value_states(&mut table);
    insert_block_name_states(&mut table);

    table
}

fn insert_start_states(table: &mut HashMap<(State, Input), State>) {
    table.insert((State::Start, Input::Character), State::ReadingIdentifier);
    table.insert((State::Start, Input::DoubleQuote), State::ReadingRawValue);
    table.insert((State::Start, Input::CurlyBracket), State::CurlyBracket);
    table.insert((State::Start, Input::Whitespace), State::Start);
    table.insert((State::Start, Input::NewLine), State::Start);
    table.insert((State::Start, Input::Comment), State::ReadingComment);
    table.insert((State::Start, Input::Digit), State::Digit);
    table.insert((State::Start, Input::Colon), State::StartBlockName);
}

fn insert_identifier_states(table: &mut HashMap<(State, Input), State>) {
    let state = State::ReadingIdentifier;
    let end_state = State::Identifier;

    // end states
    table.insert((state, Input::DoubleQuote), end_state);
    table.insert((state, Input::CurlyBracket), end_state);
    table.insert((state, Input::BackSlash), end_state);
    table.insert((state, Input::Whitespace), end_state);
    table.insert((state, Input::NewLine), end_state);
    table.insert((state, Input::Comment), end_state);
    table.insert((state, Input::Colon), end_state);
    table.insert((state, Input::Digit), end_state);

    // transitional states
    table.insert((state, Input::Character), state);
}

fn insert_comment_states(table: &mut HashMap<(State, Input), State>) {
    let state = State::ReadingComment;
    let end_state = State::Comment;

    // end state
    table.insert((state, Input::NewLine), end_state);

    // transitional states
    table.insert((state, Input::Character), state);
    table.insert((state, Input::DoubleQuote), state);
    table.insert((state, Input::CurlyBracket), state);
    table.insert((state, Input::BackSlash), state);
    table.insert((state, Input::Whitespace), state);
    table.insert((state, Input::Comment), state);
    table.insert((state, Input::Digit), state);
    table.insert((state, Input::Colon), state);
}

fn insert_raw_value_states(table: &mut HashMap<(State, Input), State>) {
    let state = State::ReadingRawValue;
    let end_state = State::RawValue;

    // end state
    table.insert((state, Input::DoubleQuote), end_state);

    // transitional states
    table.insert((state, Input::Character), state);
    table.insert((state, Input::CurlyBracket), state);
    table.insert((state, Input::BackSlash), State::ReadingEscapedChar);
    table.insert((state, Input::Whitespace), state);
    table.insert((state, Input::NewLine), state);
    table.insert((state, Input::Comment), state);
    table.insert((state, Input::Digit), state);
    table.insert((state, Input::Colon), state);
}

fn insert_escaped_char_states(table: &mut HashMap<(State, Input), State>) {
    let state = State::ReadingEscapedChar;
    let end_state = State::ReadingRawValue;
    // end states
    table.insert((state, Input::Character), end_state);
    table.insert((state, Input::DoubleQuote), end_state);
    table.insert((state, Input::CurlyBracket), end_state);
    table.insert((state, Input::BackSlash), end_state);
    table.insert((state, Input::Comment), end_state);
    table.insert((state, Input::Digit), end_state);
    table.insert((state, Input::Colon), end_state);
    table.insert((state, Input::Whitespace), end_state);
}

fn insert_block_name_states(table: &mut HashMap<(State, Input), State>) {
    // transition to read identifier
    let state = State::ReadingIdentifier;
    // anything that is not a colon is invalid and illegal, so there is nothing in the transition
    // table for it.
    table.insert((State::StartBlockName, Input::Colon), state);
    table.insert((state, Input::Character), state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_insert_start_states() {
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        let state = State::Start;
        let expected_states = vec![
            ((state, Input::Character), State::ReadingIdentifier),
            ((state, Input::Whitespace), State::Start),
            ((state, Input::NewLine), State::Start),
            ((state, Input::Colon), State::StartBlockName),
            ((state, Input::Digit), State::Digit),
            ((state, Input::Comment), State::ReadingComment),
            ((state, Input::CurlyBracket), State::CurlyBracket),
            ((state, Input::DoubleQuote), State::ReadingRawValue),
        ];
        insert_start_states(&mut table);
        for expected in expected_states.iter() {
            let (k, v) = expected;
            match table.get(k) {
                Some(s) => {
                    assert_eq!(s, v);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn should_insert_identifier_states() {
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        let state = State::ReadingIdentifier;
        let end_state = State::Identifier;
        let expected_states = vec![
            ((state, Input::Character), State::ReadingIdentifier),
            ((state, Input::Whitespace), end_state),
            ((state, Input::NewLine), end_state),
            ((state, Input::Colon), end_state),
            ((state, Input::Digit), end_state),
            ((state, Input::Comment), end_state),
            ((state, Input::CurlyBracket), end_state),
            ((state, Input::DoubleQuote), end_state),
        ];
        insert_identifier_states(&mut table);
        for expected in expected_states.iter() {
            let (k, v) = expected;
            match table.get(k) {
                Some(s) => {
                    assert_eq!(s, v);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn should_insert_comment_states() {
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        let state = State::ReadingComment;
        let end_state = State::Comment;
        let expected_states = vec![
            ((state, Input::Character), state),
            ((state, Input::Whitespace), state),
            ((state, Input::Colon), state),
            ((state, Input::Digit), state),
            ((state, Input::Comment), state),
            ((state, Input::CurlyBracket), state),
            ((state, Input::DoubleQuote), state),
            ((state, Input::NewLine), end_state),
        ];
        insert_comment_states(&mut table);
        for expected in expected_states.iter() {
            let (k, v) = expected;
            match table.get(k) {
                Some(s) => {
                    assert_eq!(s, v);
                }
                _ => {
                    panic!("Unexpected value");
                }
            }
        }
    }

    #[test]
    fn should_insert_raw_value_states() {
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        let state = State::ReadingRawValue;
        let end_state = State::RawValue;
        let expected_states = vec![
            ((state, Input::Character), state),
            ((state, Input::Whitespace), state),
            ((state, Input::Colon), state),
            ((state, Input::Digit), state),
            ((state, Input::Comment), state),
            ((state, Input::CurlyBracket), state),
            ((state, Input::NewLine), state),
            ((state, Input::DoubleQuote), end_state),
        ];
        insert_raw_value_states(&mut table);
        for expected in expected_states.iter() {
            let (k, v) = expected;
            match table.get(k) {
                Some(s) => {
                    assert_eq!(s, v);
                }
                _ => {
                    panic!("Unexpected value");
                }
            }
        }
    }
}
