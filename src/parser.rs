//Calculate first set of a grammar
#![allow(dead_code)]
use crate::Token;
// use ptree::{print_tree_with, PrintConfig, TreeBuilder};


pub struct Parser {
    input: Vec<Token>,
    current: usize,
    // parse_tree: TreeBuilder,
}

impl Parser{
    pub fn new(input: Vec<Token>) -> Self {
        Parser {
            input,
            current: 0,
            // parse_tree: TreeBuilder::new("Parse Tree".to_string()),
        }
    }

    fn peek(&mut self, token: Token) -> bool {
        if self.input[self.current] == token && self.current != self.input.len() - 1 {
            self.current += 1;
            return true;
        } else {
            return false;
        }
    }

    pub fn start(&mut self) -> bool{
        // println!("Start: {:?}", self.input[self.current]);
        if !self.statement_list(){
            println!("=============================");
            panic!("Expected statement\n =============================");
        }
        // let tree = self.parse_tree.build();

        /* let config = {
            let mut config = PrintConfig::from_env();
            config.indent = 4;
            config
        };
        let _prueba = print_tree_with(&tree, &config); */
        true
    }

    fn statement_list(&mut self) -> bool {
        // self.parse_tree.begin_child("Statement List".to_string());
        if self.input.len() - 1 == self.current{
            // println!("Statement List: {:?}", self.input[self.current]);
            return true;
        }
        // println!("Statement List: {:?}", self.input[self.current]);
        if self.statement(){
            if !self.statement_list(){
                println!("=============================");
                panic!("Expected statement\n =============================");
            }
            return true;
        }
        // self.parse_tree.begin_child("Epsilon".to_string());
        true
    }

    fn statement(&mut self) -> bool {
        // self.parse_tree.begin_child("Statement".to_string());
        // println!("Statement: {:?}", self.input[self.current]);
        if self.instruction(){
            return true;
        }
        if self.label(){
            return true;
        }
        if self.define(){
            return true;
        }
        false
    }

    fn label(&mut self) -> bool {
        // println!("Label: {:?}", self.input[self.current]);
        if self.identifier(){
            // self.parse_tree.begin_child("Label".to_string());
            // self.parse_tree.begin_child(self.input[self.current - 1].to_string());
            if self.peek(Token::Punctuation(':')){
                // self.parse_tree.begin_child(":".to_string());
                // for _ in 0..6{
                //     self.parse_tree.end_child();
                // }
                return true;
            }
            else {
                println!("=============================");
                panic!("Expected ':' After identifier: {}\n =============================", self.input[self.current - 1]);
            }
        }
        false
    }

    fn instruction(&mut self) -> bool {
        // println!("Instruction: {:?}", self.input[self.current]);
        // self.parse_tree.begin_child("Instruction".to_string());
        if self.mnemonic(){
            if self.addressing(){
                return true;
            }
            else {
                println!("=============================");
                panic!("Expected addressing mode\n =============================");
            }
        }
        false
    }

    fn mnemonic(&mut self) -> bool {
        // println!("Mnemonic: {:?}", self.input[self.current]);
        if self.peek(Token::Mnemonic("LDA".to_string())) ||
            self.peek(Token::Mnemonic("ADC".to_string())) ||
            self.peek(Token::Mnemonic("AND".to_string())) ||
            self.peek(Token::Mnemonic("LDY".to_string())) ||
            self.peek(Token::Mnemonic("ASL".to_string())) ||
            self.peek(Token::Mnemonic("BCC".to_string())) ||
            self.peek(Token::Mnemonic("BCS".to_string())) ||
            self.peek(Token::Mnemonic("BEQ".to_string())) ||
            self.peek(Token::Mnemonic("BIT".to_string())) ||
            self.peek(Token::Mnemonic("BMI".to_string())) ||
            self.peek(Token::Mnemonic("BPL".to_string())) ||
            self.peek(Token::Mnemonic("BRK".to_string())) ||
            self.peek(Token::Mnemonic("BVC".to_string())) ||
            self.peek(Token::Mnemonic("CLC".to_string())) ||
            self.peek(Token::Mnemonic("CLD".to_string())) ||
            self.peek(Token::Mnemonic("CLI".to_string())) ||
            self.peek(Token::Mnemonic("CLV".to_string())) ||
            self.peek(Token::Mnemonic("CMP".to_string())) ||
            self.peek(Token::Mnemonic("CPX".to_string())) ||
            self.peek(Token::Mnemonic("DEC".to_string())) ||
            self.peek(Token::Mnemonic("DEY".to_string())) ||
            self.peek(Token::Mnemonic("EOR".to_string())) ||
            self.peek(Token::Mnemonic("INC".to_string())) ||
            self.peek(Token::Mnemonic("INX".to_string())) ||
            self.peek(Token::Mnemonic("INY".to_string())) ||
            self.peek(Token::Mnemonic("JMP".to_string())) ||
            self.peek(Token::Mnemonic("JSR".to_string())) ||
            self.peek(Token::Mnemonic("LSR".to_string())) ||
            self.peek(Token::Mnemonic("NOP".to_string())) ||
            self.peek(Token::Mnemonic("ORA".to_string())) ||
            self.peek(Token::Mnemonic("PHP".to_string())) ||
            self.peek(Token::Mnemonic("PLP".to_string())) ||
            self.peek(Token::Mnemonic("ROL".to_string())) ||
            self.peek(Token::Mnemonic("ROR".to_string())) ||
            self.peek(Token::Mnemonic("RTI".to_string())) ||
            self.peek(Token::Mnemonic("RTS".to_string())) ||
            self.peek(Token::Mnemonic("SBC".to_string())) ||
            self.peek(Token::Mnemonic("SEC".to_string())) ||
            self.peek(Token::Mnemonic("SED".to_string())) ||
            self.peek(Token::Mnemonic("SEI".to_string())) ||
            self.peek(Token::Mnemonic("STX".to_string())) ||
            self.peek(Token::Mnemonic("STY".to_string())) ||
            self.peek(Token::Mnemonic("TAX".to_string())) ||
            self.peek(Token::Mnemonic("TAY".to_string())) ||
            self.peek(Token::Mnemonic("TSX".to_string())) ||
            self.peek(Token::Mnemonic("TXS".to_string())) ||
            self.peek(Token::Mnemonic("TYA".to_string())) ||
            self.peek(Token::Mnemonic("INY".to_string())) ||
            self.peek(Token::Mnemonic("LDX".to_string())) ||
            self.peek(Token::Mnemonic("TXA".to_string())) ||
            self.peek(Token::Mnemonic("STA".to_string())) ||
            self.peek(Token::Mnemonic("PHA".to_string())) ||
            self.peek(Token::Mnemonic("CPY".to_string())) ||
            self.peek(Token::Mnemonic("BNE".to_string())) ||
            self.peek(Token::Mnemonic("DEX".to_string())) ||
            self.peek(Token::Mnemonic("PLA".to_string())){
            // self.parse_tree.begin_child(self.input[self.current - 1].to_string());
            return true;
        }
        false
    }

    fn addressing(&mut self) -> bool {
        // println!("Addressing: {:?}", self.input[self.current]);
        if self.identifier(){
            return true;
        }

        if self.peek(Token::Punctuation('#')){
            // self.parse_tree.begin_child("#".to_string());
            if self.number(){
                if self.peek(Token::Punctuation(',')){
                    if self.addressing2(){

                    }
                    println!("=============================");
                    panic!("Expected second operand after ','\n =============================")
                }
            }
            else {
                    println!("=============================");
                panic!("Expected valid number after '#'\n =============================");
            }
        }

        if self.number(){
            if self.peek(Token::Punctuation(',')){
                if self.addressing2(){
                    return true;
                }
                else {
                    println!("=============================");
                    panic!("Expected second operand after ','\n =============================");
                }
            }
            return true;
        }

        true
    }

    fn addressing2(&mut self) -> bool {
        // println!("Addressing2: {:?}", self.input[self.current]);
        if self.addressing(){
            return true;
        }
        true
    }

    fn number(&mut self) -> bool {
        // println!("Number: {:?}", self.input[self.current]);
        if self.peek(Token::Punctuation('$')){
            // self.parse_tree.begin_child("$".to_string());
                // println!("HexNumber: {:?}", self.input[self.current]);
            match &self.input[self.current]{
                Token::HexNumber(_) => {
                    if self.current != self.input.len() - 1{
                        self.current += 1;
                    }
                    // self.parse_tree.begin_child(self.input[self.current - 1].to_string());
                    return true
                },
                _ => {
                    println!("=============================");
                    panic!("Expected HexNumber after '$'\n =============================");
                    // return false
                },
            };
        }
        match &self.input[self.current]{
            Token::HexNumber(_) => {
                if self.current != self.input.len() - 1{
                    println!("=============================");
                    panic!("Expected HexNumber after: {}\n =============================", self.input[self.current - 1]);
                }
                return false
            },
            _ => {
                return true
            },
        };
    }

    fn identifier(&mut self) -> bool {
        // println!("Identifier: {:?}", self.input[self.current]);
        match &self.input[self.current]{
            Token::Identifier(_) => {
                if self.current != self.input.len() - 1{
                    self.current += 1;
                }
                return true
            }
            _ => return false,
        }
    }

    fn define(&mut self) -> bool {
        // println!("Define: {:?}", self.input[self.current]);
        if self.peek(Token::Keyword("define".to_string())){
            match &self.input[self.current]{
                Token::Identifier(_) => {
                    if self.current != self.input.len() - 1{
                        self.current += 1;
                    }
                    if self.addressing(){
                        return true;
                    }
               }
                _ => return false,
            };
        }
        println!("=============================");
        panic!("Expected statement, {} is not a valid statement\n =============================", self.input[self.current]);
    }
}
