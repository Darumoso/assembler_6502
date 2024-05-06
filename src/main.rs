mod parser;

use crate::parser::Parser;
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
fn main() {


    let tokens_correctos_1 = vec![
        Token::Mnemonic("LDX".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(00.to_string()),
        Token::Mnemonic("LDY".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(00.to_string()),
        Token::Identifier("firstloop".to_string()),
        Token::Punctuation(':'),
        Token::Mnemonic("TXA".to_string()),
        Token::Mnemonic("STA".to_string()),
        Token::Punctuation('$'),
        Token::HexNumber(0200.to_string()),
        Token::Punctuation(','),
        Token::Identifier("Y".to_string()),
        Token::Mnemonic("PHA".to_string()),
        Token::Mnemonic("INX".to_string()),
        Token::Mnemonic("INY".to_string()),
        Token::Mnemonic("CPY".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(10.to_string()),
        Token::Mnemonic("BNE".to_string()),
        Token::Identifier("firstloop".to_string()),
        Token::Identifier("secondloop".to_string()),
        Token::Punctuation(':'),
        Token::Mnemonic("PLA".to_string()),
        Token::Mnemonic("STA".to_string()),
        Token::Punctuation('$'),
        Token::HexNumber(0200.to_string()),
        Token::Punctuation(','),
        Token::Identifier("Y".to_string()),
        Token::Mnemonic("INY".to_string()),
        Token::Mnemonic("CPY".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(20.to_string()),
        Token::Mnemonic("BNE".to_string()),
        Token::Identifier("secondloop".to_string()),
    ];


     let mut parser = Parser::new(tokens_correctos_1);
     if parser.start() {
         println!("The grammar is valid");
     }

    let tokens_correctos_2 = vec![
        Token::Mnemonic("LDX".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(08.to_string()),
        Token::Identifier("decrement".to_string()),
        Token::Punctuation(':'),
        Token::Mnemonic("DEX".to_string()),
        Token::Mnemonic("STX".to_string()),
        Token::Punctuation('$'),
        Token::HexNumber(0200.to_string()),
        Token::Mnemonic("CPX".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(03.to_string()),
        Token::Mnemonic("BNE".to_string()),
        Token::Identifier("decrement".to_string()),
        Token::Mnemonic("STX".to_string()),
        Token::Punctuation('$'),
        Token::HexNumber(0201.to_string()),
        Token::Mnemonic("BRK".to_string()),
    ];

     let mut parser = Parser::new(tokens_correctos_2);
     if parser.start() {
         println!("The grammar is valid");
     }

    let tokens_correctos_3 = vec![
        Token::Mnemonic("LDX".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber("01".to_string()),
        Token::Mnemonic("LDA".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber("aa".to_string()),
        Token::Mnemonic("STA".to_string()),
        Token::Punctuation('$'),
        Token::HexNumber("a0".to_string()),
        Token::Punctuation(','),
        Token::Identifier("X".to_string()),
        Token::Mnemonic("INX".to_string()),
        Token::Mnemonic("STA".to_string()),
        Token::Punctuation('$'),
        Token::HexNumber("a0".to_string()),
        Token::Punctuation(','),
        Token::Identifier("X".to_string()),
    ];

    let mut parser = Parser::new(tokens_correctos_3);
    if parser.start() {
        println!("The grammar is valid");
    }

    let tokens_incorrectos_1 = vec![
        Token::Mnemonic("LDY".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(00.to_string()),
        Token::Identifier("firstloop".to_string()),
        Token::Mnemonic("STA".to_string()),
        Token::Punctuation('$'),
        Token::HexNumber(0200.to_string()),
        Token::Punctuation(','),
        Token::Identifier("Y".to_string()),
    ];

    println!("\n\nThe next grammar is expected to fail\n");
    let mut parser = Parser::new(tokens_incorrectos_1);
    println!("Is the grammar valid? {}", parser.start());

    let _tokens_incorrectos_2 = vec![
        Token::Mnemonic("LDX".to_string()),
        Token::HexNumber(00.to_string()),
        Token::Mnemonic("LDY".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(00.to_string()),
        Token::Identifier("firstloop".to_string()),
        Token::Punctuation(':'),
        Token::Mnemonic("STA".to_string()),
        Token::Punctuation('$'),
        Token::HexNumber(0200.to_string()),
        Token::Punctuation(','),
        Token::Identifier("Y".to_string()),
    ];

    // let mut parser = Parser::new(tokens_incorrectos_2);
    // println!("{}", parser.start());

}


