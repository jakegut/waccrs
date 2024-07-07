use regex::Regex;
use std::error::Error;

use crate::errors::{Result, WaccrsError};

#[derive(Debug, Clone)]
pub enum Keyword {
    Int,
    Void,
    Return,
}

impl Keyword {
    pub fn from_match(s: &str) -> Option<Keyword> {
        match s {
            "int" => Some(Keyword::Int),
            "void" => Some(Keyword::Void),
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Identifier,
    Constant,
    Keyword(Keyword),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Discard,
}

#[derive(Debug, Clone)]

pub struct Token {
    token: TokenType,
    literal: String,
    line: usize,
    col: i64,
}

pub struct Lexer {
    line: usize,
    col: i64,
    input: Vec<u8>,
    regex: Vec<(TokenType, Regex)>,
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            line: 1,
            col: 1,
            input,
            tokens: vec![],
            regex: vec![
                (
                    TokenType::Identifier,
                    Regex::new(r"[a-zA-Z_]\w*\b").unwrap(),
                ),
                (TokenType::Constant, Regex::new(r"[0-9]+\b").unwrap()),
                (TokenType::OpenParen, Regex::new(r"\(").unwrap()),
                (TokenType::CloseParen, Regex::new(r"\)").unwrap()),
                (TokenType::OpenBrace, Regex::new(r"\{").unwrap()),
                (TokenType::CloseBrace, Regex::new(r"\}").unwrap()),
                (TokenType::Semicolon, Regex::new(r";").unwrap()),
                (TokenType::Discard, Regex::new(r"/[*]([^*]|([*][^/]))*[*]/").unwrap()),
                (TokenType::Discard, Regex::new(r"//[^\n\r]+").unwrap())
            ],
        }
    }

    pub fn lex(&mut self) -> Result<()> {
        while self.input.len() > 0 {
            match self.input[0] {
                b'\n' => {
                    self.line += 1;
                    self.col = 1;
                    self.input.remove(0);
                    continue;
                }
                b' ' | b'\t' => {
                    self.col += 1;
                    self.input.remove(0);
                    continue;
                }
                _ => (),
            };

            let haystack = unsafe { std::str::from_utf8_unchecked(&self.input) };

            println!("{}", haystack);

            let regexmatch = self
                .regex
                .iter()
                .filter_map(|x| {
                    if let Some(m) = x.1.find(haystack) {
                        if m.start() == 0 {
                            Some((&x.0, m))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .next()
                .ok_or("match not found")?;

            let tok = match regexmatch {
                (TokenType::Identifier, m) => {
                    let s = m.as_str();
                    if let Some(kw) = Keyword::from_match(s) {
                        Token {
                            token: TokenType::Keyword(kw),
                            literal: s.to_owned(),
                            line: self.line,
                            col: self.col,
                        }
                    } else {
                        Token {
                            token: TokenType::Identifier,
                            literal: s.to_owned(),
                            line: self.line,
                            col: self.col,
                        }
                    }
                }
                (tt, m) => Token {
                    token: tt.clone(),
                    literal: m.as_str().to_owned(),
                    line: self.line,
                    col: self.col,
                },
            };

            self.col += regexmatch.1.range().len() as i64;

            println!("{:?}: len {}", tok, self.col);

            self.input.drain(regexmatch.1.range());
            self.tokens.push(tok)
        }
        Ok(())
    }
}
