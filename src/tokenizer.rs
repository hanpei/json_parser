use std::{iter::Peekable, str::Bytes};

use crate::{error::JsonError,  JsonResult};

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
pub struct Tokenizer<'a> {
    source: Peekable<Bytes<'a>>,
    buffer: Vec<u8>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.bytes().peekable(),
            buffer: Vec::new(),
        }
    }

    fn next_byte(&mut self) -> JsonResult<u8> {
        self.source.next().ok_or(JsonError::UnexpectedEndOfJson)
    }

    pub fn next(&mut self) -> JsonResult<Token> {
        loop {
            let chr = self.next_byte()?;
            return Ok(match chr {
                b',' => Token::Comma,
                b':' => Token::Colon,
                b'[' => Token::BracketOn,
                b']' => Token::BracketOff,
                b'{' => Token::BraceOn,
                b'}' => Token::BraceOff,
                b'n' | b't' | b'f' => self.read_ident(chr)?,
                b'0'..=b'9' | b'-' => self.read_number(chr)?,
                b'"' => self.read_string()?,
                0x0A | 0x0D | 0x20 | 0x09 => continue, // whitespace '0020' ws '000A' ws '000D' ws '0009' ws
                _ => return Err(JsonError::unexpected_character(chr)),
            });
        }
    }

    fn read_ident(&mut self, ch: u8) -> JsonResult<Token> {
        match ch {
            b'n' => self.expect_str(b"ull", Token::Null),
            b't' => self.expect_str(b"rue", Token::Boolen(true)),
            b'f' => self.expect_str(b"alse", Token::Boolen(false)),
            _ => return Err(JsonError::unexpected_character(ch)),
        }
    }

    fn expect_str<T>(&mut self, str: &[u8], token: T) -> JsonResult<T> {
        for &espect in str {
            let ch = self.next_byte()?;
            if ch != espect {
                return Err(JsonError::unexpected_character(ch));
            }
        }
        Ok(token)
    }

    fn read_string(&mut self) -> JsonResult<Token> {
        self.buffer.clear();
        loop {
            let ch = self.next_byte()?;
            match ch {
                b'"' => break,
                b'\\' => self.read_escaped_chr(),
                _ => self.buffer.push(ch),
            }
        }
        match String::from_utf8(self.buffer.clone()) {
            Ok(s) => Ok(Token::String(s)),
            Err(e) => return Err(JsonError::parsing_faild(e.to_string())),
        }
    }

    //escape '"' '\' '/' 'b' 'f' 'n' 'r' 't' 'u' hex hex hex hex
    fn read_escaped_chr(&mut self) {
        // self.buffer.push(b'\\');
        if let Ok(ch) = self.next_byte() {
            match ch {
                b'b' => self.buffer.push(0x8),
                b'f' => self.buffer.push(0xC),
                b'n' => self.buffer.push(b'\n'),
                b'r' => self.buffer.push(b'\r'),
                b't' => self.buffer.push(b'\t'),
                b'u' => self.read_codepoint(),
                _ => self.buffer.push(ch),
            };
        }
    }

    fn read_hex(&mut self) -> JsonResult<u32> {
        let ch = self.next_byte()?;
        Ok(match ch {
            b'0'..=b'9' => (ch - b'0'),
            b'a'..=b'f' => (ch + 10 - b'a'),
            b'A'..=b'F' => (ch + 10 - b'A'),
            ch => return Err(JsonError::unexpected_character(ch)),
        } as u32)
    }

    fn read_codepoint(&mut self) {
        let codepoint = self.read_hex().unwrap() << 12
            | self.read_hex().unwrap() << 8
            | self.read_hex().unwrap() << 4
            | self.read_hex().unwrap();

        let ch = char::from_u32(codepoint).ok_or(JsonError::ParsingFailed("utf8".to_string()));
        let mut str = String::new();
        str.push(ch.unwrap());

        self.buffer.extend_from_slice(str.as_bytes());
    }

    fn read_number(&mut self, chr: u8) -> JsonResult<Token> {
        self.buffer.clear();
        self.buffer.push(chr);
        while let Some(&ch) = self.source.peek() {
            match ch {
                b'0'..=b'9' => self.buffer.push(ch),
                b'.' => self.buffer.push(ch),
                b'e' | b'E' => self.buffer.push(ch),
                b'+' | b'-' => self.buffer.push(ch),
                _ => break,
            }
            self.next_byte()?;
        }
        let s = String::from_utf8(self.buffer.clone()).unwrap();
        match s.parse::<f64>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_e) => return Err(JsonError::InvalidNumber),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter() {
        let s = r#"  {"a":  123,
            "b" :true,
            "c":false,
            "d":null,
            "e": {
                "f": "abca  fs  "
                "g": [1,2,3]
            },
            "h": ["x","y","z", 1,2,3]
        }  "#;
        let mut s = Tokenizer::new(s);

        while let Ok(token) = s.next() {
            println!("{:?}", token);
        }
    }

    #[test]
    fn read_string() {
        let s = r#""abc\r\n\t\b\f\\\"""#;
        let mut de = Tokenizer::new(s);
        println!("{:?}", de.next());
    }

    #[test]
    fn read_number() {
        // println!("{:?}", ret);
        assert_eq!(
            Tokenizer::new(r#" 1234 "#).next().unwrap(),
            Token::Number(1234.0)
        );
        assert_eq!(
            Tokenizer::new(r#" -1234 "#).next().unwrap(),
            Token::Number(-1234.0)
        );
        assert_eq!(
            Tokenizer::new(r#"   -1.23E4 "#).next().unwrap(),
            Token::Number(-12300.0)
        );
        assert_eq!(Tokenizer::new("1.23e4").next().unwrap(), Token::Number(12300.0));
        assert_eq!(
            Tokenizer::new("-1.23e-4").next().unwrap(),
            Token::Number(-0.000123)
        );
        assert_eq!(
            Tokenizer::new("-1.23e+4").next().unwrap(),
            Token::Number(-12300.0)
        );
        assert_eq!(
            Tokenizer::new(r#"   -1.23e"#).next().err().unwrap(),
            JsonError::InvalidNumber
        );
    }

    #[test]
    fn temp() {
        // '0020' ws '000A' ws '000D' ws '0009' ws
        let a = '\u{6c49}';

        println!("{:?}", a);
        println!("{:?}", "6c49".bytes());
        println!("{:?}", "0".bytes());
        println!("{:?}", "a".bytes());
        println!("{:?}", char::from_u32(99));
        let a = char::from_u32(0x6c49);
        println!("{:?}", a);

        // println!("{:?}",'\u{000A}');
        // println!("{:?}",'\u{000D}');
        // println!("{:?}",'\u{0009}');
    }
}
