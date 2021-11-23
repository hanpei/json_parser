use std::fmt::Error;

use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum JsonError {
    UnexpectedToken(String),
    UnexpectedEndOfJson,
    InvalidType(String),
    UndefinedField(String),
    UnexpectedCharacter(char),
    InvalidNumber,
    ParsingFailed(String),
}

impl JsonError {
    pub fn unexpected_token(token: Token) -> Self {
        JsonError::UnexpectedToken(format!("{:?}", token))
    }

    pub fn invalid_type(typ: String) -> Self {
        JsonError::InvalidType(typ.into())
    }

    pub fn undefined_field(field: String) -> Self {
        JsonError::UndefinedField(field.into())
    }

    pub fn unexpected_character(byte: u8) -> Self {
        JsonError::UnexpectedCharacter(char::from_u32(byte as u32).unwrap_or('?'))
    }

    pub fn parsing_faild(err: String) -> Self {
        JsonError::ParsingFailed(err)
    }
}
