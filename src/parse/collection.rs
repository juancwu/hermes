use crate::api::Collection;
use crate::parse::lexer::Lexer;
use crate::parse::token::Token;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEndOfTokens,
    UnexpectedToken(Token),
    UnexpectedCollectionKeyword,
}

#[derive(Debug, Default)]
pub struct CollectionParser {
    pending_resolutions: u32,
    pending_environment: String,
    resolved_environments: Vec<String>,
    env_status_cache: bool,
    collection: Collection,
}

impl CollectionParser {
    pub fn parse(&mut self, lexer: &mut Lexer) -> Result<Collection, ParseError> {
        // reset
        self.collection = Collection::default();
        while let Some(token) = lexer.next_token() {
            println!("{:?}", token);
            match token {
                Token::CurlyLeft | Token::CurlyRight => {}
                Token::CollectionBlock => {
                    let mut block_start = false;
                    while let Some(token) = lexer.next_token() {
                        match token {
                            Token::Identifier(ref ident) => {
                                match self.parse_collection_identifier(ident.as_str(), lexer) {
                                    Err(e) => return Err(e),
                                    _ => {}
                                }
                            }
                            Token::BlockIdentifier(ref ident) => {
                                self.collection.set_identifier(ident.clone());
                            }
                            Token::CurlyLeft => {
                                if block_start {
                                    return Err(ParseError::UnexpectedToken(token));
                                }
                                block_start = true;
                            }
                            Token::CurlyRight => {
                                if !block_start {
                                    return Err(ParseError::UnexpectedToken(token));
                                }
                                // finish reading collection block
                                break;
                            }
                            _ => {
                                return Err(ParseError::UnexpectedToken(token));
                            }
                        }
                    }
                }
                Token::EnvironmentBlock => {
                    let mut block_start = false;
                    let mut env_ident = self.random_env_identifier();
                    match lexer.next_token() {
                        Some(token) => match token {
                            Token::BlockIdentifier(ref ident) => {
                                env_ident = ident.clone();
                            }
                            _ => {}
                        },
                        _ => {
                            return Err(ParseError::UnexpectedEndOfTokens);
                        }
                    }

                    // create new environment
                    self.collection.new_environment(env_ident.clone());

                    // check if this environment is pending
                    if self.pending_environment == env_ident {
                        // if the cached status of the environment is enabled, then we should set
                        // it to enabled. the status is set back to disabled when the env is
                        // pending
                        if self.env_status_cache {
                            self.collection.enable_active_environment();
                        }
                        self.collection.set_active_environment(env_ident.clone());
                    }

                    while let Some(token) = lexer.next_token() {
                        match token {
                            Token::Identifier(key) => match self.get_raw_value(lexer) {
                                Ok(value) => {
                                    self.collection.add_environment_entry(key, value);
                                }
                                Err(e) => return Err(e),
                            },
                            Token::CurlyLeft => {
                                if block_start {
                                    return Err(ParseError::UnexpectedToken(token));
                                }
                                block_start = true;
                            }
                            Token::CurlyRight => {
                                if !block_start {
                                    return Err(ParseError::UnexpectedToken(token));
                                }
                                // finish reading collection block
                                break;
                            }
                            _ => {
                                return Err(ParseError::UnexpectedToken(token));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(self.collection.clone())
    }

    fn parse_collection_identifier(
        &mut self,
        ident: &str,
        lexer: &mut Lexer,
    ) -> Result<(), ParseError> {
        match ident {
            "name" => {
                let raw_value = match self.get_raw_value(lexer) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                self.collection.set_name(raw_value);
            }
            "include" => {
                let raw_value = match self.get_raw_value(lexer) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                // println!("Include: {}", raw_value);
            }
            "environment" => {
                // get the status of the environment
                let env_enabled = match self.get_environment_status(lexer) {
                    Ok(b) => b,
                    Err(e) => return Err(e),
                };
                // cached for later use
                self.env_status_cache = env_enabled;
                let block_ident = match self.get_identifier(lexer) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                };
                if !self.is_environment_resolved(&block_ident) {
                    self.pending_resolutions += 1;
                    self.pending_environment = block_ident;
                    self.collection.disable_active_environment();
                } else {
                    self.collection.set_active_environment(block_ident);
                    if env_enabled {
                        self.collection.enable_active_environment();
                    }
                }
            }
            _ => {
                return Err(ParseError::UnexpectedCollectionKeyword);
            }
        }
        Ok(())
    }

    fn get_environment_status(&self, lexer: &mut Lexer) -> Result<bool, ParseError> {
        match lexer.next_token() {
            Some(token) => match token {
                Token::StateEnabled => Ok(true),
                Token::StateDisabled => Ok(false),
                _ => Err(ParseError::UnexpectedToken(token)),
            },
            None => Err(ParseError::UnexpectedEndOfTokens),
        }
    }

    fn get_raw_value(&self, lexer: &mut Lexer) -> Result<String, ParseError> {
        match lexer.next_token() {
            Some(token) => match token {
                Token::RawValue(ref value) => Ok(value.clone()),
                _ => Err(ParseError::UnexpectedToken(token)),
            },
            _ => Err(ParseError::UnexpectedEndOfTokens),
        }
    }

    fn get_identifier(&self, lexer: &mut Lexer) -> Result<String, ParseError> {
        match lexer.next_token() {
            Some(token) => match token {
                Token::Identifier(ref ident) => Ok(ident.clone()),
                _ => Err(ParseError::UnexpectedToken(token)),
            },
            _ => Err(ParseError::UnexpectedEndOfTokens),
        }
    }

    fn is_environment_resolved(&self, name: &String) -> bool {
        for env in self.resolved_environments.iter() {
            if env == name {
                return true;
            }
        }
        false
    }

    fn random_env_identifier(&self) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect()
    }
}
