use std::collections::HashMap;

/// ! Missing entries in the transition table
/// ! means that the (State, Input) combination results in State::Error

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
    Start,

    ReadIdentifier,
    EndIdentifier,

    ExpectSubBlockType,
    ReadSubBlockType,
    EndSubBlockType,

    EndDigit,

    // Read a block identifier which starts with two colons
    ExpectFollowUpColon,
    ExpectIdentifier,

    // String value that starts with a tilt
    ReadString,
    EndString,

    ReadEscapedCharacter,

    // delimeters
    EndLeftCurlyBrace,
    EndRightCurlyBrace,

    Error,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Input {
    NewLine,
    Whitespace,
    Character,
    LeftCurlyBrace,
    RightCurlyBrace,
    Tilt,
    Backslash,
    Digit,
    Colon,
    Dot,
    Other,
}

/// Match the given character with an Input type to use with a transition table.
pub fn char_to_input(ch: char) -> Input {
    match ch {
        ' ' | '\t' => Input::Whitespace,
        '\n' => Input::NewLine,
        'a'..='z' | 'A'..'Z' | '-' => Input::Character,
        '{' => Input::LeftCurlyBrace,
        '}' => Input::RightCurlyBrace,
        '`' => Input::Tilt,
        '\\' => Input::Backslash,
        '0'..='9' => Input::Digit,
        ':' => Input::Colon,
        '.' => Input::Dot,
        _ => Input::Other,
    }
}

/// Builds a transition table to use to create lexemes.
pub fn build_transition_table() -> HashMap<(State, Input), State> {
    let mut table: HashMap<(State, Input), State> = HashMap::new();

    insert_start_states(&mut table);
    insert_read_identifier_states(&mut table);
    insert_expect_sub_block_type_states(&mut table);
    insert_read_sub_block_type_states(&mut table);
    insert_expect_follow_up_colon_states(&mut table);
    insert_expect_identifier_states(&mut table);
    insert_read_string_states(&mut table);
    insert_read_escaped_character_states(&mut table);

    table
}

pub fn is_transitional_state(state: State) -> bool {
    match state {
        State::Start
        | State::ReadIdentifier
        | State::ReadSubBlockType
        | State::ReadString
        | State::ReadEscapedCharacter
        | State::ExpectSubBlockType
        | State::ExpectFollowUpColon
        | State::ExpectIdentifier => true,
        _ => false,
    }
}

fn insert_start_states(table: &mut HashMap<(State, Input), State>) {
    table.insert((State::Start, Input::Character), State::ReadIdentifier);
    table.insert((State::Start, Input::Dot), State::ExpectSubBlockType);
    table.insert((State::Start, Input::Digit), State::EndDigit);
    table.insert((State::Start, Input::Colon), State::ExpectFollowUpColon);
    table.insert((State::Start, Input::Tilt), State::ReadString);
    // initial white spaces
    table.insert((State::Start, Input::Whitespace), State::Start);
    table.insert((State::Start, Input::NewLine), State::Start);
    // delimeters
    table.insert(
        (State::Start, Input::LeftCurlyBrace),
        State::EndLeftCurlyBrace,
    );
    table.insert(
        (State::Start, Input::RightCurlyBrace),
        State::EndRightCurlyBrace,
    );
}

fn insert_read_identifier_states(table: &mut HashMap<(State, Input), State>) {
    // loop back state
    table.insert(
        (State::ReadIdentifier, Input::Character),
        State::ReadIdentifier,
    );
    table.insert((State::ReadIdentifier, Input::Dot), State::EndIdentifier);
    table.insert((State::ReadIdentifier, Input::Digit), State::EndIdentifier);
    table.insert((State::ReadIdentifier, Input::Colon), State::EndIdentifier);
    table.insert((State::ReadIdentifier, Input::Tilt), State::EndIdentifier);
    // initial white spaces
    table.insert(
        (State::ReadIdentifier, Input::Whitespace),
        State::EndIdentifier,
    );
    table.insert(
        (State::ReadIdentifier, Input::NewLine),
        State::EndIdentifier,
    );
    // delimeters
    table.insert(
        (State::ReadIdentifier, Input::LeftCurlyBrace),
        State::EndIdentifier,
    );
    table.insert(
        (State::ReadIdentifier, Input::RightCurlyBrace),
        State::EndIdentifier,
    );
    table.insert(
        (State::ReadIdentifier, Input::Backslash),
        State::EndIdentifier,
    );
    table.insert((State::ReadIdentifier, Input::Other), State::EndIdentifier);
}

fn insert_expect_sub_block_type_states(table: &mut HashMap<(State, Input), State>) {
    table.insert(
        (State::ExpectSubBlockType, Input::Character),
        State::ReadSubBlockType,
    );
}

fn insert_read_sub_block_type_states(table: &mut HashMap<(State, Input), State>) {
    // loop back state
    table.insert(
        (State::ReadSubBlockType, Input::Character),
        State::ReadSubBlockType,
    );
    table.insert(
        (State::ReadSubBlockType, Input::Dot),
        State::EndSubBlockType,
    );
    table.insert(
        (State::ReadSubBlockType, Input::Digit),
        State::EndSubBlockType,
    );
    table.insert(
        (State::ReadSubBlockType, Input::Colon),
        State::EndSubBlockType,
    );
    table.insert(
        (State::ReadSubBlockType, Input::Tilt),
        State::EndSubBlockType,
    );
    // initial white spaces
    table.insert(
        (State::ReadSubBlockType, Input::Whitespace),
        State::EndSubBlockType,
    );
    table.insert(
        (State::ReadSubBlockType, Input::NewLine),
        State::EndSubBlockType,
    );
    // delimeters
    table.insert(
        (State::ReadSubBlockType, Input::LeftCurlyBrace),
        State::EndSubBlockType,
    );
    table.insert(
        (State::ReadSubBlockType, Input::RightCurlyBrace),
        State::EndSubBlockType,
    );
}

fn insert_expect_follow_up_colon_states(table: &mut HashMap<(State, Input), State>) {
    table.insert(
        (State::ExpectFollowUpColon, Input::Colon),
        State::ExpectIdentifier,
    );
}

fn insert_expect_identifier_states(table: &mut HashMap<(State, Input), State>) {
    table.insert(
        ((State::ExpectIdentifier, Input::Character)),
        State::ReadIdentifier,
    );
}

fn insert_read_string_states(table: &mut HashMap<(State, Input), State>) {
    table.insert((State::ReadString, Input::Character), State::ReadString);
    table.insert((State::ReadString, Input::Digit), State::ReadString);
    table.insert(
        (State::ReadString, Input::LeftCurlyBrace),
        State::ReadString,
    );
    table.insert(
        (State::ReadString, Input::RightCurlyBrace),
        State::ReadString,
    );
    table.insert((State::ReadString, Input::Whitespace), State::ReadString);
    table.insert((State::ReadString, Input::NewLine), State::ReadString);
    table.insert((State::ReadString, Input::Tilt), State::EndString);
    table.insert((State::ReadString, Input::Colon), State::ReadString);
    table.insert((State::ReadString, Input::Dot), State::ReadString);
    table.insert((State::ReadString, Input::Other), State::ReadString);
    table.insert(
        (State::ReadString, Input::Backslash),
        State::ReadEscapedCharacter,
    );
}

fn insert_read_escaped_character_states(table: &mut HashMap<(State, Input), State>) {
    table.insert(
        (State::ReadEscapedCharacter, Input::Character),
        State::ReadString,
    );
    table.insert((State::ReadEscapedCharacter, Input::Dot), State::ReadString);
    table.insert(
        (State::ReadEscapedCharacter, Input::Digit),
        State::ReadString,
    );
    table.insert(
        (State::ReadEscapedCharacter, Input::Colon),
        State::ReadString,
    );
    table.insert(
        (State::ReadEscapedCharacter, Input::Tilt),
        State::ReadString,
    );
    // initial white spaces
    table.insert(
        (State::ReadEscapedCharacter, Input::Whitespace),
        State::ReadString,
    );
    table.insert(
        (State::ReadEscapedCharacter, Input::NewLine),
        State::ReadString,
    );
    // delimeters
    table.insert(
        (State::ReadEscapedCharacter, Input::LeftCurlyBrace),
        State::ReadString,
    );
    table.insert(
        (State::ReadEscapedCharacter, Input::RightCurlyBrace),
        State::ReadString,
    );
    table.insert(
        (State::ReadEscapedCharacter, Input::Backslash),
        State::ReadString,
    );
    table.insert(
        (State::ReadEscapedCharacter, Input::Other),
        State::ReadString,
    );
}

#[cfg(test)]
mod tests {
    use core::panic;

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
                    panic!(
                        "Unexpected state for {:?}. Expecting {:?}",
                        key, expected_state
                    );
                }
            }
        }
    }

    #[test]
    fn should_check_state_is_transitional() {
        let states = vec![
            // transitional
            (State::Start, true),
            (State::ReadIdentifier, true),
            (State::ReadEscapedCharacter, true),
            (State::ReadString, true),
            (State::ReadSubBlockType, true),
            (State::ExpectIdentifier, true),
            (State::ExpectFollowUpColon, true),
            (State::ExpectSubBlockType, true),
            // non-transitional
            (State::EndLeftCurlyBrace, false),
            (State::EndRightCurlyBrace, false),
            (State::EndDigit, false),
            (State::EndIdentifier, false),
            (State::EndSubBlockType, false),
            (State::EndString, false),
        ];
        for pair in states {
            let (state, expected) = pair;
            let result = is_transitional_state(state);
            assert_eq!(
                expected, result,
                "Is state {:?} transitional? Expected: {}, Got: {}",
                state, expected, result
            );
        }
    }

    #[test]
    fn should_insert_start_states() {
        let expected = vec![
            ((State::Start, Input::Character), State::ReadIdentifier),
            ((State::Start, Input::Dot), State::ExpectSubBlockType),
            ((State::Start, Input::Digit), State::EndDigit),
            ((State::Start, Input::Colon), State::ExpectFollowUpColon),
            ((State::Start, Input::Tilt), State::ReadString),
            // initial white spaces
            ((State::Start, Input::Whitespace), State::Start),
            ((State::Start, Input::NewLine), State::Start),
            // delimeters
            (
                (State::Start, Input::LeftCurlyBrace),
                State::EndLeftCurlyBrace,
            ),
            (
                (State::Start, Input::RightCurlyBrace),
                State::EndRightCurlyBrace,
            ),
        ];
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        insert_start_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_read_identifier_states() {
        let expected = vec![
            // loop back state
            (
                (State::ReadIdentifier, Input::Character),
                State::ReadIdentifier,
            ),
            ((State::ReadIdentifier, Input::Dot), State::EndIdentifier),
            ((State::ReadIdentifier, Input::Digit), State::EndIdentifier),
            ((State::ReadIdentifier, Input::Colon), State::EndIdentifier),
            ((State::ReadIdentifier, Input::Tilt), State::EndIdentifier),
            // initial white spaces
            (
                (State::ReadIdentifier, Input::Whitespace),
                State::EndIdentifier,
            ),
            (
                (State::ReadIdentifier, Input::NewLine),
                State::EndIdentifier,
            ),
            // delimeters
            (
                (State::ReadIdentifier, Input::LeftCurlyBrace),
                State::EndIdentifier,
            ),
            (
                (State::ReadIdentifier, Input::RightCurlyBrace),
                State::EndIdentifier,
            ),
            (
                (State::ReadIdentifier, Input::Backslash),
                State::EndIdentifier,
            ),
            ((State::ReadIdentifier, Input::Other), State::EndIdentifier),
        ];
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        insert_read_identifier_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_expect_sub_block_type_states() {
        let expected = vec![(
            (State::ExpectSubBlockType, Input::Character),
            State::ReadSubBlockType,
        )];
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        insert_expect_sub_block_type_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_sub_block_type_states() {
        let expected = vec![
            // loop back state
            (
                (State::ReadSubBlockType, Input::Character),
                State::ReadSubBlockType,
            ),
            (
                (State::ReadSubBlockType, Input::Dot),
                State::EndSubBlockType,
            ),
            (
                (State::ReadSubBlockType, Input::Digit),
                State::EndSubBlockType,
            ),
            (
                (State::ReadSubBlockType, Input::Colon),
                State::EndSubBlockType,
            ),
            (
                (State::ReadSubBlockType, Input::Tilt),
                State::EndSubBlockType,
            ),
            // initial white spaces
            (
                (State::ReadSubBlockType, Input::Whitespace),
                State::EndSubBlockType,
            ),
            (
                (State::ReadSubBlockType, Input::NewLine),
                State::EndSubBlockType,
            ),
            // delimeters
            (
                (State::ReadSubBlockType, Input::LeftCurlyBrace),
                State::EndSubBlockType,
            ),
            (
                (State::ReadSubBlockType, Input::RightCurlyBrace),
                State::EndSubBlockType,
            ),
        ];
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        insert_read_sub_block_type_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_expect_follow_up_colon_states() {
        let expected = vec![(
            // single valid state, everything else causes an error
            (State::ExpectFollowUpColon, Input::Colon),
            State::ExpectIdentifier,
        )];
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        insert_expect_follow_up_colon_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_expect_identifier_states() {
        let expected = vec![(
            // single valid state, everything else causes an error
            (State::ExpectIdentifier, Input::Character),
            State::ReadIdentifier,
        )];
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        insert_expect_identifier_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_read_string_states() {
        let expected = vec![
            ((State::ReadString, Input::Character), State::ReadString),
            ((State::ReadString, Input::Digit), State::ReadString),
            (
                (State::ReadString, Input::LeftCurlyBrace),
                State::ReadString,
            ),
            (
                (State::ReadString, Input::RightCurlyBrace),
                State::ReadString,
            ),
            ((State::ReadString, Input::Whitespace), State::ReadString),
            ((State::ReadString, Input::NewLine), State::ReadString),
            ((State::ReadString, Input::Tilt), State::EndString),
            ((State::ReadString, Input::Colon), State::ReadString),
            ((State::ReadString, Input::Dot), State::ReadString),
            ((State::ReadString, Input::Other), State::ReadString),
            (
                (State::ReadString, Input::Backslash),
                State::ReadEscapedCharacter,
            ),
        ];
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        insert_read_string_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_read_escaped_character_states() {
        let expected = vec![
            (
                (State::ReadEscapedCharacter, Input::Character),
                State::ReadString,
            ),
            ((State::ReadEscapedCharacter, Input::Dot), State::ReadString),
            (
                (State::ReadEscapedCharacter, Input::Digit),
                State::ReadString,
            ),
            (
                (State::ReadEscapedCharacter, Input::Colon),
                State::ReadString,
            ),
            (
                (State::ReadEscapedCharacter, Input::Tilt),
                State::ReadString,
            ),
            // initial white spaces
            (
                (State::ReadEscapedCharacter, Input::Whitespace),
                State::ReadString,
            ),
            (
                (State::ReadEscapedCharacter, Input::NewLine),
                State::ReadString,
            ),
            // delimeters
            (
                (State::ReadEscapedCharacter, Input::LeftCurlyBrace),
                State::ReadString,
            ),
            (
                (State::ReadEscapedCharacter, Input::RightCurlyBrace),
                State::ReadString,
            ),
            (
                (State::ReadEscapedCharacter, Input::Backslash),
                State::ReadString,
            ),
            (
                (State::ReadEscapedCharacter, Input::Other),
                State::ReadString,
            ),
        ];
        let mut table: HashMap<(State, Input), State> = HashMap::new();
        insert_read_escaped_character_states(&mut table);
        verify_result(&table, expected);
    }
}
