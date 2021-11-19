use crate::tokenizer::Token;

#[derive(Debug,PartialEq)]
pub enum JsonError {
    UnexpectedToken(String),
    UnexpectedEndOfJson,
    InvalidType(String),
    UndefinedField(String),
    UnexpectedCharacter(char),
    InvalidNumber,
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
}
