use std::{
    collections::HashMap,
    io::{self, Read},
};

use thiserror::Error;
use utf8_reader::Utf8Reader;

macro_rules! test {
    ($e:expr; $name:path) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err($name(err))),
        }
    };
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(err) => return Some(Err(err)),
        }
    };
}

#[derive(Debug, Error)]
pub enum LexError {
    #[error("failed to read")]
    ReadError(#[from] io::Error),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Equal,
    Ampersand,
}

pub struct Lexer<T: Read> {
    reader: Utf8Reader<T>,
    peek: Option<char>,
}

impl<T: Read> Lexer<T> {
    pub fn new(reader: T) -> Self {
        Self {
            reader: Utf8Reader::new(reader),
            peek: None,
        }
    }

    fn next_char(&mut self) -> Option<Result<char, LexError>> {
        if let Some(p) = self.peek {
            self.peek = None;
            return Some(Ok(p));
        }

        let c = test!(self.reader.next()?; LexError::from);
        Some(Ok(c))
    }

    fn peek_char(&mut self) -> Option<Result<char, LexError>> {
        if let Some(p) = self.peek {
            return Some(Ok(p));
        }

        let c = test!(self.reader.next()?; LexError::from);
        self.peek = Some(c);
        Some(Ok(c))
    }
}

impl<T: Read> Iterator for Lexer<T> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = test!(self.next_char()?);

        match c {
            '&' => Some(Ok(Token::Ampersand)),
            '=' => Some(Ok(Token::Equal)),
            c => {
                let mut ident = String::from(c);
                loop {
                    let p = test!(match self.peek_char() {
                        Some(t) => t,
                        None => break,
                    });

                    if p == '&' || p == '=' {
                        break;
                    }

                    let c = test!(self.next_char()?);
                    ident.push(c);
                }

                Some(Ok(Token::Identifier(ident)))
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("failed to lex")]
    LexError(#[from] LexError),
    #[error("invalid token (expected {expected:?}, found {found:?})")]
    InvalidToken { expected: Token, found: Token },
    #[error("unexpected end of file")]
    UnexpectedEof,
}

pub fn parse(reader: impl Read) -> Result<HashMap<String, String>, ParseError> {
    let mut parser = Parser {
        lexer: Lexer::new(reader),
        peek: None,
    };

    parser.parse()
}

struct Parser<T: Read> {
    lexer: Lexer<T>,
    peek: Option<Token>,
}

macro_rules! parse {
    ($e:expr) => {
        match $e {
            Some(r) => match r {
                Ok(t) => t,
                Err(e) => return Err(e),
            },
            None => return Err(ParseError::UnexpectedEof),
        }
    };
}

impl<T: Read> Parser<T> {
    fn next_token(&mut self) -> Option<Result<Token, ParseError>> {
        // NOTE: clone here since Token does not implement Copy
        if let Some(p) = self.peek.clone() {
            self.peek = None;
            return Some(Ok(p));
        }

        let t = test!(self.lexer.next()?; ParseError::from);
        Some(Ok(t))
    }

    fn peek_token(&mut self) -> Option<Result<Token, ParseError>> {
        if let Some(p) = self.peek.clone() {
            return Some(Ok(p));
        }

        let t = test!(self.lexer.next()?; ParseError::from);
        self.peek = Some(t.clone());
        Some(Ok(t))
    }

    fn has_next_token(&mut self) -> bool {
        let p = self.peek_token();

        p.is_some()
    }

    fn parse_ident(&mut self) -> Result<String, ParseError> {
        let key = parse!(self.next_token());

        match key {
            Token::Identifier(i) => Ok(i),
            t => Err(ParseError::InvalidToken {
                expected: Token::Identifier("".to_string()),
                found: t,
            }),
        }
    }

    fn parse_eq(&mut self) -> Result<(), ParseError> {
        let token = parse!(self.next_token());

        match token {
            Token::Equal => Ok(()),
            t => Err(ParseError::InvalidToken {
                expected: Token::Equal,
                found: t,
            }),
        }
    }

    fn parse_amp(&mut self) -> Result<(), ParseError> {
        let token = parse!(self.next_token());

        match token {
            Token::Ampersand => Ok(()),
            t => Err(ParseError::InvalidToken {
                expected: Token::Ampersand,
                found: t,
            }),
        }
    }

    fn parse_kv(&mut self) -> Result<(String, String), ParseError> {
        let key = self.parse_ident()?;
        self.parse_eq()?;
        let value = self.parse_ident()?;

        Ok((key, value))
    }

    fn parse(&mut self) -> Result<HashMap<String, String>, ParseError> {
        let mut out = HashMap::new();

        if self.has_next_token() {
            let (k, v) = self.parse_kv()?;
            out.insert(k, v);

            while self.has_next_token() {
                self.parse_amp()?;

                let (k, v) = self.parse_kv()?;
                out.insert(k, v);
            }
        }

        Ok(out)
    }
}

pub fn encode(data: &HashMap<String, String>) -> String {
    let mut entries = Vec::with_capacity(data.len());

    for (k, v) in data {
        entries.push(format!("{}={}", escape(k), escape(v)));
    }

    entries.join("&")
}

fn escape(data: &str) -> String {
    data.replace("&", "").replace("=", "")
}
