use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum State {
    Start,
    Id,
    IdEnd,
    SBT,
    SBTPre,
    SBTEnd,
    FirstColon,
    DoubleColon,
    LeftCurlyBrace,
    RightCurlyBrace,
    Digit,
    Str,
    StrPre,
    StrEnd,
    EscStr,
    RawStrPrePre,
    RawStrPre,
    RawStr,
    Comment,
    CommentEnd,
    Error,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Input {
    Character,
    NewLine,
    Whitespace,
    DoubleQuote,
    LeftCurlyBrace,
    RightCurlyBrace,
    Hashtag,
    Backslash,
    Digit,
    Colon,
    Dot,
    Underscore,
    Error,
}

/// Match the given character with an Input type to use with a transition table.
pub fn char_to_input(ch: char) -> Input {
    match ch {
        ' ' | '\t' => Input::Whitespace,
        '\n' => Input::NewLine,
        'a'..='z' | 'A'..'Z' | '-' => Input::Character,
        '"' => Input::DoubleQuote,
        '_' => Input::Underscore,
        '{' => Input::LeftCurlyBrace,
        '}' => Input::RightCurlyBrace,
        '#' => Input::Hashtag,
        '\\' => Input::Backslash,
        '0'..='9' => Input::Digit,
        ':' => Input::Colon,
        '.' => Input::Dot,
        _ => Input::Error,
    }
}

/// Builds a transition table to use to create lexemes.
pub fn build_transition_table() -> HashMap<(State, Input), State> {
    let mut table: HashMap<(State, Input), State> = HashMap::new();

    insert_start_states(&mut table);
    insert_id_states(&mut table);
    insert_sub_block_pre_type_states(&mut table);
    insert_sub_block_type_states(&mut table);

    table
}

pub fn is_transitional_state(state: State) -> bool {
    match state {
        State::Start => true,
        State::Id => true,
        State::SBT => true,
        State::SBTPre => true,
        State::Str => true,
        State::StrPre => true,
        State::EscStr => true,
        State::RawStrPrePre => true,
        State::RawStrPre => true,
        State::Comment => true,
        _ => false,
    }
}

fn insert_start_states(table: &mut HashMap<(State, Input), State>) {
    table.insert((State::Start, Input::Character), State::Id);
    table.insert((State::Start, Input::NewLine), State::Start);
    table.insert((State::Start, Input::Whitespace), State::Start);
    table.insert((State::Start, Input::DoubleQuote), State::StrPre);
    table.insert((State::Start, Input::LeftCurlyBrace), State::LeftCurlyBrace);
    table.insert(
        (State::Start, Input::RightCurlyBrace),
        State::RightCurlyBrace,
    );
    table.insert((State::Start, Input::Hashtag), State::Comment);
    table.insert((State::Start, Input::Backslash), State::Error);
    table.insert((State::Start, Input::Digit), State::Digit);
    table.insert((State::Start, Input::Colon), State::FirstColon);
    table.insert((State::Start, Input::Dot), State::SBTPre);
    table.insert((State::Start, Input::Underscore), State::RawStrPrePre);
    table.insert((State::Start, Input::Error), State::Error);
}

fn insert_id_states(table: &mut HashMap<(State, Input), State>) {
    table.insert((State::Id, Input::Character), State::Id);
    table.insert((State::Id, Input::NewLine), State::IdEnd);
    table.insert((State::Id, Input::Whitespace), State::IdEnd);
    table.insert((State::Id, Input::DoubleQuote), State::IdEnd);
    table.insert((State::Id, Input::LeftCurlyBrace), State::IdEnd);
    table.insert((State::Id, Input::RightCurlyBrace), State::IdEnd);
    table.insert((State::Id, Input::Hashtag), State::IdEnd);
    table.insert((State::Id, Input::Digit), State::IdEnd);
    table.insert((State::Id, Input::Colon), State::IdEnd);
    table.insert((State::Id, Input::Dot), State::IdEnd);
    table.insert((State::Id, Input::Underscore), State::IdEnd);
    table.insert((State::Id, Input::Error), State::Error);
}

fn insert_sub_block_pre_type_states(table: &mut HashMap<(State, Input), State>) {
    table.insert((State::SBTPre, Input::Character), State::SBT);
    table.insert((State::SBTPre, Input::NewLine), State::Error);
    table.insert((State::SBTPre, Input::Whitespace), State::Error);
    table.insert((State::SBTPre, Input::DoubleQuote), State::Error);
    table.insert((State::SBTPre, Input::LeftCurlyBrace), State::Error);
    table.insert((State::SBTPre, Input::RightCurlyBrace), State::Error);
    table.insert((State::SBTPre, Input::Hashtag), State::Error);
    table.insert((State::SBTPre, Input::Digit), State::Error);
    table.insert((State::SBTPre, Input::Colon), State::Error);
    table.insert((State::SBTPre, Input::Dot), State::Error);
    table.insert((State::SBTPre, Input::Underscore), State::Error);
    table.insert((State::SBTPre, Input::Error), State::Error);
}

fn insert_sub_block_type_states(table: &mut HashMap<(State, Input), State>) {
    table.insert((State::SBT, Input::Character), State::SBT);
    table.insert((State::SBT, Input::NewLine), State::SBTEnd);
    table.insert((State::SBT, Input::Whitespace), State::SBTEnd);
    table.insert((State::SBT, Input::DoubleQuote), State::SBTEnd);
    table.insert((State::SBT, Input::LeftCurlyBrace), State::SBTEnd);
    table.insert((State::SBT, Input::RightCurlyBrace), State::SBTEnd);
    table.insert((State::SBT, Input::Hashtag), State::SBTEnd);
    table.insert((State::SBT, Input::Digit), State::SBTEnd);
    table.insert((State::SBT, Input::Colon), State::SBTEnd);
    table.insert((State::SBT, Input::Dot), State::SBTEnd);
    table.insert((State::SBT, Input::Underscore), State::SBTEnd);
    table.insert((State::SBT, Input::Error), State::Error);
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
                    assert_eq!(state, expected_state);
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
    fn should_insert_start_states() {
        let mut table = HashMap::new();
        let expected = vec![
            ((State::Start, Input::Character), State::Id),
            ((State::Start, Input::NewLine), State::Start),
            ((State::Start, Input::Whitespace), State::Start),
            ((State::Start, Input::DoubleQuote), State::StrPre),
            ((State::Start, Input::LeftCurlyBrace), State::LeftCurlyBrace),
            (
                (State::Start, Input::RightCurlyBrace),
                State::RightCurlyBrace,
            ),
            ((State::Start, Input::Hashtag), State::Comment),
            ((State::Start, Input::Backslash), State::Error),
            ((State::Start, Input::Digit), State::Digit),
            ((State::Start, Input::Colon), State::FirstColon),
            ((State::Start, Input::Dot), State::SBTPre),
            ((State::Start, Input::Underscore), State::RawStrPrePre),
            ((State::Start, Input::Error), State::Error),
        ];
        insert_start_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_id_states() {
        let mut table = HashMap::new();
        let expected = vec![
            ((State::Id, Input::Character), State::Id),
            ((State::Id, Input::NewLine), State::IdEnd),
            ((State::Id, Input::Whitespace), State::IdEnd),
            ((State::Id, Input::DoubleQuote), State::IdEnd),
            ((State::Id, Input::LeftCurlyBrace), State::IdEnd),
            ((State::Id, Input::RightCurlyBrace), State::IdEnd),
            ((State::Id, Input::Hashtag), State::IdEnd),
            ((State::Id, Input::Digit), State::IdEnd),
            ((State::Id, Input::Colon), State::IdEnd),
            ((State::Id, Input::Dot), State::IdEnd),
            ((State::Id, Input::Underscore), State::IdEnd),
            ((State::Id, Input::Error), State::Error),
        ];
        insert_id_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_sub_block_pre_type_states() {
        let mut table = HashMap::new();
        let expected = vec![
            ((State::SBTPre, Input::Character), State::SBT),
            ((State::SBTPre, Input::NewLine), State::Error),
            ((State::SBTPre, Input::Whitespace), State::Error),
            ((State::SBTPre, Input::DoubleQuote), State::Error),
            ((State::SBTPre, Input::LeftCurlyBrace), State::Error),
            ((State::SBTPre, Input::RightCurlyBrace), State::Error),
            ((State::SBTPre, Input::Hashtag), State::Error),
            ((State::SBTPre, Input::Digit), State::Error),
            ((State::SBTPre, Input::Colon), State::Error),
            ((State::SBTPre, Input::Dot), State::Error),
            ((State::SBTPre, Input::Underscore), State::Error),
            ((State::SBTPre, Input::Error), State::Error),
        ];
        insert_sub_block_pre_type_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_insert_sub_block_type_states() {
        let mut table = HashMap::new();
        let expected = vec![
            ((State::SBT, Input::Character), State::SBT),
            ((State::SBT, Input::NewLine), State::SBTEnd),
            ((State::SBT, Input::Whitespace), State::SBTEnd),
            ((State::SBT, Input::DoubleQuote), State::SBTEnd),
            ((State::SBT, Input::LeftCurlyBrace), State::SBTEnd),
            ((State::SBT, Input::RightCurlyBrace), State::SBTEnd),
            ((State::SBT, Input::Hashtag), State::SBTEnd),
            ((State::SBT, Input::Digit), State::SBTEnd),
            ((State::SBT, Input::Colon), State::SBTEnd),
            ((State::SBT, Input::Dot), State::SBTEnd),
            ((State::SBT, Input::Underscore), State::SBTEnd),
            ((State::SBT, Input::Error), State::Error),
        ];
        insert_sub_block_type_states(&mut table);
        verify_result(&table, expected);
    }

    #[test]
    fn should_return_true_for_transitional_states() {
        let values = vec![
            (State::Start, true),
            (State::Id, true),
            (State::IdEnd, false),
            (State::SBT, true),
            (State::SBTPre, true),
            (State::SBTEnd, false),
            (State::FirstColon, false),
            (State::DoubleColon, false),
            (State::LeftCurlyBrace, false),
            (State::RightCurlyBrace, false),
            (State::Digit, false),
            (State::Str, true),
            (State::StrPre, true),
            (State::StrEnd, false),
            (State::EscStr, true),
            (State::RawStrPrePre, true),
            (State::RawStrPre, true),
            (State::RawStr, false),
            (State::Comment, true),
            (State::CommentEnd, false),
            (State::Error, false),
        ];

        for value in values {
            let (state, expected) = value;
            let result = is_transitional_state(state);
            assert_eq!(expected, result, "Testing state {:?}", state);
        }
    }
}
