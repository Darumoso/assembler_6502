#![allow(dead_code)]
//use regex::Regex;
use std::fmt;

#[derive(Debug, PartialEq, Clone,)]
pub enum Token {
    Mnemonic(String),
    Keyword(String),
    HexNumber(String),
    Identifier(String),
    Punctuation(char),
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Mnemonic(s) => write!(f, "{}", s),
            Token::Keyword(s) => write!(f, "{}", s),
            Token::HexNumber(n) => write!(f, "{}", n),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Punctuation(c) => write!(f, "{}", c),
        }
    }
}

/* #[derive(Debug)]
pub struct Lexer {
    input: String,
    current: usize,
    column: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    //Creates a new lexer
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            current: 0,
            column: 1,
            line: 1,
            tokens: Vec::new(),
        }
    }

    //Lexes the input
    pub fn lex(&mut self) -> Vec<Token> {
        self.clear_comments();
        println!("Input:\n{}", self.input);
        while self.current < self.input.len() {
            let c = self.input.chars().nth(self.current).unwrap();
            match c {
                'A'..='Z' | 'a'..='z' => {
                    Token::Mnemonic(sel);
                }
                '0'..='9' => {
                    self.lex_number();
                }
                '$' => {
                    self.lex_hex_number();
                }
                '#' => {
                    self.lex_punctuation('#');
                }
                ':' => {
                    self.lex_punctuation(':');
                }
                ',' => {
                    self.lex_punctuation(',');
                }
                _ => {
                    self.lex_punctuation(c);
                }
            }
        }
        self.tokens.clone()
    }

    //Skips comments
    fn clear_comments(&mut self){
        let re = Regex::new(r";.*(\n)").unwrap();
        self.input = re.replace_all(&self.input, "").to_string();
        self.input = self.input.trim().to_string();
    }

} */
