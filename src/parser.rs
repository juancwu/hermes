use std::{collections::HashMap, fs, path::PathBuf};
use walkdir::WalkDir;

use crate::lexer::{Lexer, Token};

struct BlockField {
    identifier: String,
    enabled: bool,
    value: String,
}

impl BlockField {
    pub fn new(identifier: String, enabled: bool, value: String) -> Self {
        BlockField {
            identifier,
            enabled,
            value,
        }
    }
}

struct Block {
    identifier: String,
    block_type: String,
    sub_block_type: String,
    fields: Vec<BlockField>,
}

impl Block {
    pub fn new(
        block_type: String,
        sub_block_type: String,
        identifier: String,
        fields: Vec<BlockField>,
    ) -> Self {
        Block {
            block_type,
            sub_block_type,
            identifier,
            fields,
        }
    }

    pub fn add_field(&mut self, field: BlockField) {
        self.fields.push(field);
    }
}

pub fn parse(dir: &str) {
    let hermes_files = get_hermes_files(dir);

    let mut lexer: Lexer;
    for hermes_file in hermes_files {
        let contents = match read_file_contents(&hermes_file) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!(
                    "Error reading hermes file {}: {}",
                    hermes_file.display(),
                    err
                );
                continue;
            }
        };
        let symbol_table: HashMap<String, String> = HashMap::new();
        let mut blocks: Vec<Block> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        lexer = Lexer::new(&contents);
        while let Some(t) = lexer.next_token() {
            tokens.push(t);
        }
        let mut current_token_idx = 0;
        let mut current_block_idx = 0;
        while current_token_idx < tokens.len() {
            let t = tokens[current_token_idx].clone();
            match t {
                Token::BlockType(block_type) => {
                    let mut next_idx = if current_token_idx + 1 >= tokens.len() {
                        // TODO: clean up and log error message
                        // last token, break out of loop
                        break;
                    } else {
                        current_token_idx + 1
                    };
                    let sub_block_type = match tokens[next_idx].clone() {
                        Token::SubBlockType(sub_block_type) => {
                            current_token_idx = current_token_idx + 1;
                            sub_block_type
                        }
                        _ => String::new(),
                    };
                    next_idx = if current_token_idx + 1 >= tokens.len() {
                        // TODO: clean up and log error message
                        // last token, break out of loop
                        break;
                    } else {
                        current_token_idx + 1
                    };
                    let identifier = match tokens[next_idx].clone() {
                        Token::Identifier(identifier) => {
                            current_token_idx = current_token_idx + 1;
                            identifier
                        }
                        _ => String::new(),
                    };
                    let block = Block::new(block_type, sub_block_type, identifier, Vec::new());
                    blocks.push(block);
                }
                Token::Delimeter(d) if d == '{' => {
                    current_block_idx = blocks.len() - 1;
                }
                Token::Identifier(identifier) => {
                    let mut next_idx = if current_token_idx + 1 >= tokens.len() {
                        break;
                    } else {
                        current_token_idx + 1
                    };
                    let enabled = match tokens[next_idx].clone() {
                        Token::Digit(d) => {
                            current_token_idx = current_token_idx + 1;
                            d == 1
                        }
                        _ => {
                            // TODO: error state, expecting a status
                            false
                        }
                    };
                    next_idx = if current_token_idx + 1 >= tokens.len() {
                        break;
                    } else {
                        current_token_idx + 1
                    };
                    let value = match tokens[next_idx].clone() {
                        Token::StringValue(s) => {
                            current_token_idx = current_token_idx + 1;
                            s
                        }
                        Token::Identifier(id) => {
                            match symbol_table.get(&id) {
                                Some(v) => v.clone(),
                                None => {
                                    // TODO: pending resolution
                                    // add to pending identifier
                                    String::new()
                                }
                            }
                        }
                        _ => {
                            // TODO: error state
                            String::new()
                        }
                    };
                    blocks[current_block_idx]
                        .add_field(BlockField::new(identifier, enabled, value));
                }
                _ => {}
            }
        }
    }

    // let mut lexer = Lexer::new(input);
    // // for _ in 0..6 {
    // //     println!("{:?}", lexer.next_token());
    // // }
    // let mut symbol_table = HashMap::<String, Token>::new();
    // let blocks = vec![];
}

fn get_hermes_files(dir: &str) -> Vec<PathBuf> {
    let mut hermes_files = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hermes") {
            if let Ok(abs_path) = fs::canonicalize(path) {
                hermes_files.push(abs_path);
            }
        }
    }
    hermes_files
}

fn read_file_contents(file_path: &PathBuf) -> std::io::Result<String> {
    fs::read_to_string(file_path)
}
