use crate::ast::Ident;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    String(String),
    Ident(Ident),
    Nil
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen,
    Value(Value),
    RParen,
    EOF,
    LCurly,
    RCurly,
    LBracket,
    RBracket
}

impl Token {
    pub fn guess_value(word: &str) -> Self {
        if let Ok(word_int) = word.trim().parse::<i32>() {
            Token::Value(Value::Int(word_int))
        } else if (word.chars().nth(0).unwrap() == '"') && (word.chars().last().unwrap() == '"') {
            let without_quotes = &word[1..word.len() - 1];
            Token::Value(Value::String(without_quotes.to_string()))
        } else {
            Token::Value(Value::Ident(word.to_string()))
        }
    }
}
