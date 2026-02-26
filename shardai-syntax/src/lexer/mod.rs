use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::lexer::token::{Token, TokenType};

mod token;

#[derive(Debug)]
pub enum LexLiteral {
    Number(f64),
    String(String),
    Nil
}

#[derive(Clone, Debug)]
pub struct LexError {
    line: usize,
    message: String
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] {}", self.line, self.message)
    }
}

impl Error for LexError {}

pub struct Lexer {
    tkn_start: usize,
    source_idx: usize,
    source: String,

    line: usize,
    keywords: HashMap<String, TokenType>
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let keywords: HashMap<String, TokenType> = HashMap::from([
            ("var".into(), TokenType::Var)
        ]);

        Self {
            source,
            keywords,
            line: 0,
            source_idx: 0,
            tkn_start: 0
        }
    }

    // Lexer entrypoint
    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.tkn_start = self.source_idx;

            if let Some(t) = self.lex_token()? {
                tokens.push(t)
            }
        }

        Ok(tokens)
    }

    // Lex methods
    fn lex_token(&mut self) -> Result<Option<Token>, LexError> {
        let mut literal: Option<LexLiteral> = None;

        let c = self.advance()
            .expect("Lexer out of bounds (this should have been checked)");
        let token = match c {
            '=' => Some(TokenType::Equals),
            ';' => Some(TokenType::Semicolon),

            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            },

            // not a symbol
            _ => {
                if self.is_digit(c) {
                    let number = self.number()?;

                    literal = Some(LexLiteral::Number(number));
                    Some(TokenType::Number)
                } else if self.is_letter(c) {
                    let (identifier, token_type) = self.identifier()?;

                    literal = Some(LexLiteral::String(identifier));
                    Some(token_type)
                } else {
                    return Err(LexError {
                        line: self.line,
                        message: "Unknown character".into()
                    })
                }
            }
        };

        if let Some(token_type) = token {
            let lexeme = self.source[self.tkn_start..self.source_idx].into();
            let length = self.source_idx - self.tkn_start;

            Ok(Some(Token { start: self.tkn_start, lexeme, token_type, length, literal }))
        } else {
            Ok(None)
        }
    }

    // Lex helpers
    fn consume_digits(&mut self) {
        while self.peek().map_or(false, |c| self.is_digit(c)) {
            self.advance();
        }
    }

    fn number(&mut self) -> Result<f64, LexError> {
        // first part of number
        self.consume_digits();

        // decimal part
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| self.is_digit(c)) {
            self.advance(); // consume "."
            self.consume_digits();
        }

        let num_str = &self.source[self.tkn_start..self.source_idx];
        num_str.parse::<f64>().map_err(|_| LexError {
            line: self.line,
            message: "Malformed number".into()
        })
    }

    fn consume_letters(&mut self) {
        while self.peek().map_or(false, |c| self.is_letter(c)) {
            self.advance();
        }
    }

    fn identifier(&mut self) -> Result<(String, TokenType), LexError> {
        self.consume_letters();

        let word = self.source[self.tkn_start..self.source_idx].to_string();
        if let Some(kw) = self.keywords.get(&word) {
            Ok((word, kw.clone()))
        } else {
            Ok((word, TokenType::Identifier))
        }
    }

    // Utils
    fn is_at_end(&self) -> bool {
        self.source_idx >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.source_idx)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.source_idx + 1)
    }

    fn advance(&mut self) -> Option<char> {
        let last = self.peek();
        self.source_idx += 1;

        last
    }

    // Type checkers
    fn is_letter(&self, character: char) -> bool {
        matches!(character, 'a'..='z' | 'A'..='Z' | '_')
    }

    fn is_digit(&self, character: char) -> bool {
        matches!(character, '0'..='9')
    }
}