/* use std::env;
use std::fs; */

mod lexer;
mod parser;

//use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::lexer::Token;

fn main() {
    /* let input = fs::read_to_string(env::args().nth(1).expect("Expected file argument"))
        .expect("Failed to read file"); */

    /* let mut lexer = Lexer::new(input);
    let tokens = lexer.lex();
    println!("{:?}", tokens); */

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
    println!("{}", parser.start());

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
    println!("{}", parser.start());

    let tokens_correctos_3 = vec![
        Token::Mnemonic("LDX".to_string()),
        Token::Punctuation('#'),
        Token::Punctuation('$'),
        Token::HexNumber(01.to_string()),
        Token::Mnemonic("LDA".to_string()),
        Token::Punctuation('#'),
        // Token::Punctuation('$'),
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
    println!("{}", parser.start());


    let _tokens_incorrectos_1 = vec![
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

    // let mut parser = Parser::new(tokens_incorrectos_1);
    // println!("{}", parser.start());

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


