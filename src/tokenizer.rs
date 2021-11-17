use std::{iter::Peekable, str::Chars};

use crate::{JsonResult, error::JsonError};

#[derive(Debug, PartialEq)]
pub enum Token {
    Comma,          // ,
    Colon,          // :
    BracketOn,      // [
    BracketOff,     // ]
    BraceOn,        // {
    BraceOff,       // }
    String(String), // "string"
    Number(f64),    // 123
    Boolen(bool),   // "true/false"
    Null,           // "null"
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn next(&mut self) -> JsonResult<Token> {
        while let Some(ch) = self.source.next() {
            return Ok(match ch {
                ',' => Token::Comma,
                ':' => Token::Colon,
                '[' => Token::BracketOn,
                ']' => Token::BracketOff,
                '{' => Token::BraceOn,
                '}' => Token::BraceOff,
                '0'..='9' => Token::Number(self.read_number(ch)?),
                '"' => Token::String(self.read_string(ch)),
                't' | 'f' | 'n' => {
                    let symbol = self.read_symbol(ch);
                    match symbol.as_str() {
                        "true" => Token::Boolen(true),
                        "false" => Token::Boolen(false),
                        "null" => Token::Null,
                        _ => return Err(JsonError::UnexpectedToken(symbol)),
                    }
                }

                _ => {
                    if ch.is_whitespace() {
                        continue;
                    } else {
                        return Err(JsonError::UnexpectedCharacter(ch));
                    }
                }
            });
        }
        Err(JsonError::UnexpectedEndOfJson)
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source: source.chars().peekable(),
        }
    }

    fn read_number(&mut self, ch: char) -> JsonResult<f64> {
        let mut value = ch.to_string();
        let mut point = false;
        while let Some(&n) = self.source.peek() {
            match n {
                '0'..='9' => {
                    value.push(n);
                    self.source.next();
                }
                '.' => {
                    if !point {
                        point = true;
                        value.push(n);
                        self.source.next();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        match value.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(_e) => Err(JsonError::UnexpectedToken("wrong number".to_string())),
        }
    }

    fn read_string(&mut self, first: char) -> String {
        let mut ret = String::new();
        let mut escape = false;

        while let Some(ch) = self.source.next() {
            if ch == first && escape == false {
                return ret;
            }
            match ch {
                '\\' => {
                    if escape {
                        escape = false;
                        ret.push(ch);
                    } else {
                        escape = true;
                    }
                }
                _ => {
                    ret.push(ch);
                    escape = false;
                }
            }
        }

        return ret;
    }

    fn read_symbol(&mut self, ch: char) -> String {
        let mut symbol = String::from(ch).to_string();

        while let Some(&c) = self.source.peek() {
            match c {
                'a'..='z' => {
                    symbol.push(c);
                    self.source.next();
                }
                _ => break,
            }
        }
        symbol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let a = "abcd";
        let mut t = Tokenizer::new(a);

        assert_eq!(t.source.peek(), Some(&'a'));
        t.source.next();
        assert_eq!(t.source.peek(), Some(&'b'));
        t.source.next();
        assert_eq!(t.source.peek(), Some(&'c'));
    }

    #[test]
    fn read_string() {
        let mut str = Tokenizer::new(r#""abc n123 ""#);

        let mut ret = String::new();
        while let Some(s) = str.source.next() {
            ret = str.read_string(s);
        }
        assert_eq!(ret, "abc n123 ");
    }

    #[test]
    fn read_symbol() {
        let mut str = Tokenizer::new("true");
        let mut ret = String::new();
        while let Some(s) = str.source.next() {
            ret = str.read_symbol(s);
        }
        assert_eq!(ret, "true");
    }

    #[test]
    fn read_number() {
        let mut str = Tokenizer::new("123.4");
        let mut ret = 0.0;
        while let Some(s) = str.source.next() {
            ret = str.read_number(s).unwrap();
        }
        assert_eq!(ret, 123.4);
    }

    #[test]
    #[should_panic]
    fn read_number_parse_err() {
        let mut str = Tokenizer::new("1v23.4x");
        let mut ret = 0.0;
        while let Some(s) = str.source.next() {
            ret = str.read_number(s).unwrap();
        }
        assert_eq!(ret, 123.4);
    }

    #[test]
    fn iterator() {
        let s = r#",     :[]{} true false null"#;
        let mut t = Tokenizer::new(s);
        let mut next = t.next().unwrap();
        assert_eq!(Token::Comma, next);
        next = t.next().unwrap();
        assert_eq!(Token::Colon, next);
        next = t.next().unwrap();
        assert_eq!(Token::BracketOn, next);
        next = t.next().unwrap();
        assert_eq!(Token::BracketOff, next);
        next = t.next().unwrap();
        assert_eq!(Token::BraceOn, next);
        next = t.next().unwrap();
        assert_eq!(Token::BraceOff, next);
        next = t.next().unwrap();
        assert_eq!(Token::Boolen(true), next);
        next = t.next().unwrap();
        assert_eq!(Token::Boolen(false), next);
        next = t.next().unwrap();
        assert_eq!(Token::Null, next);
    }

    #[test]
    #[should_panic]
    fn iter_error() {
        let s = r#"tru"#;
        let mut t = Tokenizer::new(s);
        let next = t.next().unwrap();
        assert_eq!(Token::Comma, next);
    }
}
