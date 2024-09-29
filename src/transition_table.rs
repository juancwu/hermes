use std::{collections::HashMap, slice::Iter};

/// ! Missing entries in the transition table
/// ! means that the (State, Input) combination results in State::Error

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
    /// Start state, this is where the FSM will decide which direction it will go to.
    Start,

    /// An identifier acts just like any identifier in any programming language. Since there is
    /// also block types, reading them its just like reading an identifier and later the literal is
    /// matched against a list of block types which will decide whether it is just a normal
    /// identifier or a block type.
    ReadIdentifier,
    /// The end state when reading an identifier.
    EndIdentifier,

    /// This states would captured the same stuff as ReadIdentifier but includes the initial "."
    /// which will make it able to differentiate between reserve keywords/identifiers for sub block
    /// types in the literal pattern matching.
    ReadSubBlockType,
    /// The end state when reading a sub block type.
    EndSubBlockType,

    /// A special identifier is an identifier wrapped in double quotes which allows spaces and
    /// start with numbers which is not allowed in normal identifiers.
    ReadSpecialIdentifier,
    /// The end state when reading a special identifier.
    EndSpecialIdentifier,

    /// The can only be single digits in the Hermes language so right from the Start state when
    /// a digit is encountered, it goes to the end state to extract the literal.
    EndDigit,

    /// String value that starts with a tilt and ends with a tilt. A string value allows multiple
    /// lines.
    ReadString,
    /// The end state when reading a string value.
    EndString,

    /// This is a special state that will accept any character after a backslash when reading
    /// a string value. There is no end state because once the next character is read, it will just
    /// go back to complete the read on the string value.
    ReadEscapedCharacter,

    /// Delimeters are single character entries, just like digits, they not need any intermediate
    /// state to complete the read.
    EndDelimeter,

    /// End of File state
    EOF,

    /// Error state, something unknown or unexpected happened during a read.
    Error,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Input {
    NewLine,
    Whitespace,
    Character,
    Phiten,
    Underscore,
    Delimeter,
    Dot,
    Tilt,
    Backslash,
    Digit,
    DoubleQuote,
    Other,
    EOF,
}

impl Input {
    pub fn iterator() -> Iter<'static, Input> {
        static INPUTS: [Input; 13] = [
            Input::NewLine,
            Input::Whitespace,
            Input::Character,
            Input::Phiten,
            Input::Underscore,
            Input::Delimeter,
            Input::Dot,
            Input::Tilt,
            Input::Backslash,
            Input::Digit,
            Input::DoubleQuote,
            Input::Other,
            Input::EOF,
        ];
        INPUTS.iter()
    }
}

/// Match the given character with an Input type to use with a transition table.
pub fn char_to_input(ch: char) -> Input {
    match ch {
        ' ' | '\t' => Input::Whitespace,
        '\n' => Input::NewLine,
        'a'..='z' | 'A'..'Z' => Input::Character,
        '-' => Input::Phiten,
        '_' => Input::Underscore,
        '{' | '}' => Input::Delimeter,
        '.' => Input::Dot,
        '`' => Input::Tilt,
        '\\' => Input::Backslash,
        '0'..='9' => Input::Digit,
        '"' => Input::DoubleQuote,
        '\0' => Input::EOF,
        _ => Input::Other,
    }
}

/// Builds a transition table to use to create lexemes.
pub fn build_transition_table() -> HashMap<(State, Input), State> {
    let mut table: HashMap<(State, Input), State> = HashMap::new();

    insert_start_states(&mut table);
    insert_read_identifier_states(&mut table);
    insert_read_special_identifier_states(&mut table);
    insert_read_string_states(&mut table);
    insert_read_escaped_character_states(&mut table);
    insert_read_sub_block_type_states(&mut table);

    table
}

pub fn is_transitional_state(state: State) -> bool {
    match state {
        State::Start
        | State::ReadIdentifier
        | State::ReadSubBlockType
        | State::ReadSpecialIdentifier
        | State::ReadString
        | State::ReadEscapedCharacter => true,
        _ => false,
    }
}

fn insert_start_states(table: &mut HashMap<(State, Input), State>) {
    for input in Input::iterator() {
        let next_state = match input {
            Input::NewLine => State::Start,
            Input::Whitespace => State::Start,
            Input::Character => State::ReadIdentifier,
            Input::Underscore => State::ReadIdentifier,
            Input::Delimeter => State::EndDelimeter,
            Input::Dot => State::ReadSubBlockType,
            Input::Digit => State::EndDigit,
            Input::DoubleQuote => State::ReadSpecialIdentifier,
            Input::Phiten => State::Error,
            Input::Backslash => State::Error,
            Input::Tilt => State::ReadString,
            Input::EOF => State::EOF,
            Input::Other => State::Error,
        };
        table.insert((State::Start, *input), next_state);
    }
}

fn insert_read_identifier_states(table: &mut HashMap<(State, Input), State>) {
    for input in Input::iterator() {
        let next_state = match input {
            Input::NewLine => State::EndIdentifier,
            Input::Whitespace => State::EndIdentifier,
            Input::Character => State::ReadIdentifier,
            Input::Underscore => State::ReadIdentifier,
            Input::Delimeter => State::EndIdentifier,
            Input::Dot => State::EndIdentifier,
            Input::Digit => State::ReadIdentifier,
            Input::DoubleQuote => State::EndIdentifier,
            Input::Phiten => State::ReadIdentifier,
            Input::Backslash => State::EndIdentifier,
            Input::Tilt => State::EndIdentifier,
            Input::EOF => State::EndIdentifier,
            Input::Other => State::EndIdentifier,
        };
        table.insert((State::ReadIdentifier, *input), next_state);
    }
}

fn insert_read_special_identifier_states(table: &mut HashMap<(State, Input), State>) {
    for input in Input::iterator() {
        let next_state = match input {
            Input::NewLine => State::EndSpecialIdentifier,
            Input::Whitespace => State::ReadSpecialIdentifier,
            Input::Character => State::ReadSpecialIdentifier,
            Input::Underscore => State::ReadSpecialIdentifier,
            Input::Delimeter => State::ReadSpecialIdentifier,
            Input::Dot => State::ReadSpecialIdentifier,
            Input::Digit => State::ReadSpecialIdentifier,
            Input::DoubleQuote => State::EndSpecialIdentifier,
            Input::Phiten => State::ReadSpecialIdentifier,
            Input::Backslash => State::ReadSpecialIdentifier,
            Input::Tilt => State::ReadSpecialIdentifier,
            Input::EOF => State::EndSpecialIdentifier,
            Input::Other => State::ReadSpecialIdentifier,
        };
        table.insert((State::ReadSpecialIdentifier, *input), next_state);
    }
}
fn insert_read_string_states(table: &mut HashMap<(State, Input), State>) {
    for input in Input::iterator() {
        let next_state = match input {
            Input::NewLine => State::ReadString,
            Input::Whitespace => State::ReadString,
            Input::Character => State::ReadString,
            Input::Underscore => State::ReadString,
            Input::Delimeter => State::ReadString,
            Input::Dot => State::ReadString,
            Input::Digit => State::ReadString,
            Input::DoubleQuote => State::ReadString,
            Input::Phiten => State::ReadString,
            Input::Backslash => State::ReadEscapedCharacter,
            Input::Tilt => State::EndString,
            Input::EOF => State::EndString,
            Input::Other => State::ReadString,
        };
        table.insert((State::ReadString, *input), next_state);
    }
}

fn insert_read_escaped_character_states(table: &mut HashMap<(State, Input), State>) {
    for input in Input::iterator() {
        let next_state = match input {
            Input::NewLine => State::ReadString,
            Input::Whitespace => State::ReadString,
            Input::Character => State::ReadString,
            Input::Underscore => State::ReadString,
            Input::Delimeter => State::ReadString,
            Input::Dot => State::ReadString,
            Input::Digit => State::ReadString,
            Input::DoubleQuote => State::ReadString,
            Input::Phiten => State::ReadString,
            Input::Backslash => State::ReadString,
            Input::Tilt => State::ReadString,
            Input::EOF => State::EndString,
            Input::Other => State::ReadString,
        };
        table.insert((State::ReadEscapedCharacter, *input), next_state);
    }
}

fn insert_read_sub_block_type_states(table: &mut HashMap<(State, Input), State>) {
    for input in Input::iterator() {
        let next_state = match input {
            Input::NewLine => State::EndSubBlockType,
            Input::Whitespace => State::EndSubBlockType,
            Input::Character => State::ReadSubBlockType,
            Input::Underscore => State::ReadSubBlockType,
            Input::Delimeter => State::EndSubBlockType,
            Input::Dot => State::EndSubBlockType,
            Input::Digit => State::ReadSubBlockType,
            Input::DoubleQuote => State::EndSubBlockType,
            Input::Phiten => State::ReadSubBlockType,
            Input::Backslash => State::EndSubBlockType,
            Input::Tilt => State::EndSubBlockType,
            Input::EOF => State::EndSubBlockType,
            Input::Other => State::EndSubBlockType,
        };
        table.insert((State::ReadSubBlockType, *input), next_state);
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::vec;

    use super::*;

    fn verify_result(
        table: &HashMap<(State, Input), State>,
        expected: Vec<((State, Input), State)>,
    ) {
        for tuple in expected.iter() {
            let (key, expected_state) = tuple;
            match table.get(key) {
                Some(state) => {
                    assert_eq!(
                        state, expected_state,
                        "Expected state {:?}, received state {:?}",
                        expected_state, state
                    );
                }
                None => {
                    panic!("No state for {:?}. Expecting {:?}", key, expected_state);
                }
            }
        }
    }

    #[test]
    fn should_determine_right_transitional_state() {
        let test_cases = vec![
            (State::Start, true),
            (State::ReadIdentifier, true),
            (State::EndIdentifier, false),
            (State::ReadSubBlockType, true),
            (State::EndSubBlockType, false),
            (State::ReadString, true),
            (State::EndString, false),
            (State::ReadEscapedCharacter, true),
            (State::ReadSpecialIdentifier, true),
            (State::EndSpecialIdentifier, false),
            (State::EndDelimeter, false),
            (State::EndDigit, false),
            (State::EOF, false),
            (State::Error, false),
        ];
        for case in test_cases {
            let (state, expected) = case;
            let result = is_transitional_state(state);
            assert_eq!(
                expected, result,
                "Failed to determine correct transitional state. Expecting {}, received: {}",
                expected, result
            );
        }
    }

    #[test]
    fn should_insert_start_states() {
        let mut states: Vec<((State, Input), State)> = Vec::new();
        let state = State::Start;
        for input in Input::iterator() {
            let next_state = match input {
                Input::NewLine => State::Start,
                Input::Whitespace => State::Start,
                Input::Character => State::ReadIdentifier,
                Input::Underscore => State::ReadIdentifier,
                Input::Delimeter => State::EndDelimeter,
                Input::Dot => State::ReadSubBlockType,
                Input::Digit => State::EndDigit,
                Input::DoubleQuote => State::ReadSpecialIdentifier,
                Input::Phiten => State::Error,
                Input::Backslash => State::Error,
                Input::Tilt => State::ReadString,
                Input::EOF => State::EOF,
                Input::Other => State::Error,
            };
            states.push(((state, *input), next_state));
        }
        let mut table = HashMap::new();
        insert_start_states(&mut table);
        verify_result(&table, states);
    }

    #[test]
    fn should_insert_read_identifier_states() {
        let mut states: Vec<((State, Input), State)> = Vec::new();
        let state = State::ReadIdentifier;
        for input in Input::iterator() {
            let next_state = match input {
                Input::NewLine => State::EndIdentifier,
                Input::Whitespace => State::EndIdentifier,
                Input::Character => State::ReadIdentifier,
                Input::Underscore => State::ReadIdentifier,
                Input::Delimeter => State::EndIdentifier,
                Input::Dot => State::EndIdentifier,
                Input::Digit => State::ReadIdentifier,
                Input::DoubleQuote => State::EndIdentifier,
                Input::Phiten => State::ReadIdentifier,
                Input::Backslash => State::EndIdentifier,
                Input::Tilt => State::EndIdentifier,
                Input::EOF => State::EndIdentifier,
                Input::Other => State::EndIdentifier,
            };
            states.push(((state, *input), next_state));
        }
        let mut table = HashMap::new();
        insert_read_identifier_states(&mut table);
        verify_result(&table, states);
    }

    #[test]
    fn should_insert_read_special_identifier_states() {
        let mut states: Vec<((State, Input), State)> = Vec::new();
        let state = State::ReadSpecialIdentifier;
        for input in Input::iterator() {
            let next_state = match input {
                Input::NewLine => State::EndSpecialIdentifier,
                Input::Whitespace => State::ReadSpecialIdentifier,
                Input::Character => State::ReadSpecialIdentifier,
                Input::Underscore => State::ReadSpecialIdentifier,
                Input::Delimeter => State::ReadSpecialIdentifier,
                Input::Dot => State::ReadSpecialIdentifier,
                Input::Digit => State::ReadSpecialIdentifier,
                Input::DoubleQuote => State::EndSpecialIdentifier,
                Input::Phiten => State::ReadSpecialIdentifier,
                Input::Backslash => State::ReadSpecialIdentifier,
                Input::Tilt => State::ReadSpecialIdentifier,
                Input::EOF => State::EndSpecialIdentifier,
                Input::Other => State::ReadSpecialIdentifier,
            };
            states.push(((state, *input), next_state));
        }
        let mut table = HashMap::new();
        insert_read_special_identifier_states(&mut table);
        verify_result(&table, states);
    }

    #[test]
    fn should_insert_read_string_states() {
        let mut states: Vec<((State, Input), State)> = Vec::new();
        let state = State::ReadString;
        for input in Input::iterator() {
            let next_state = match input {
                Input::NewLine => State::ReadString,
                Input::Whitespace => State::ReadString,
                Input::Character => State::ReadString,
                Input::Underscore => State::ReadString,
                Input::Delimeter => State::ReadString,
                Input::Dot => State::ReadString,
                Input::Digit => State::ReadString,
                Input::DoubleQuote => State::ReadString,
                Input::Phiten => State::ReadString,
                Input::Backslash => State::ReadEscapedCharacter,
                Input::Tilt => State::EndString,
                Input::EOF => State::EndString,
                Input::Other => State::ReadString,
            };
            states.push(((state, *input), next_state));
        }
        let mut table = HashMap::new();
        insert_read_string_states(&mut table);
        verify_result(&table, states);
    }

    #[test]
    fn should_insert_read_escaped_character_states() {
        let mut states: Vec<((State, Input), State)> = Vec::new();
        let state = State::ReadEscapedCharacter;
        for input in Input::iterator() {
            let next_state = match input {
                Input::NewLine => State::ReadString,
                Input::Whitespace => State::ReadString,
                Input::Character => State::ReadString,
                Input::Underscore => State::ReadString,
                Input::Delimeter => State::ReadString,
                Input::Dot => State::ReadString,
                Input::Digit => State::ReadString,
                Input::DoubleQuote => State::ReadString,
                Input::Phiten => State::ReadString,
                Input::Backslash => State::ReadString,
                Input::Tilt => State::ReadString,
                Input::EOF => State::EndString,
                Input::Other => State::ReadString,
            };
            states.push(((state, *input), next_state));
        }
        let mut table = HashMap::new();
        insert_read_escaped_character_states(&mut table);
        verify_result(&table, states);
    }

    #[test]
    fn should_insert_read_sub_block_type_states() {
        let mut states: Vec<((State, Input), State)> = Vec::new();
        let state = State::ReadSubBlockType;
        for input in Input::iterator() {
            let next_state = match input {
                Input::NewLine => State::EndSubBlockType,
                Input::Whitespace => State::EndSubBlockType,
                Input::Character => State::ReadSubBlockType,
                Input::Underscore => State::ReadSubBlockType,
                Input::Delimeter => State::EndSubBlockType,
                Input::Dot => State::EndSubBlockType,
                Input::Digit => State::ReadSubBlockType,
                Input::DoubleQuote => State::EndSubBlockType,
                Input::Phiten => State::ReadSubBlockType,
                Input::Backslash => State::EndSubBlockType,
                Input::Tilt => State::EndSubBlockType,
                Input::EOF => State::EndSubBlockType,
                Input::Other => State::EndSubBlockType,
            };
            states.push(((state, *input), next_state));
        }
        let mut table = HashMap::new();
        insert_read_sub_block_type_states(&mut table);
        verify_result(&table, states);
    }
}
