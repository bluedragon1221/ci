use std::rc::Rc;

use crate::parsers::CIEvalError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Int(i32),
    String(String), // "var"
    Symbol(String), // var
    Ident(String), // 'var
    True,
    Nil
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen,
    Value(Value),
    RParen,
    EOF,
    Fn,
    LCurly,
    RCurly,
    LBracket,
    RBracket,
}

impl Token {
    pub fn guess_value(word: &str) -> Self {
        if let Ok(word_int) = word.trim().parse::<i32>() {
            Token::Value(Value::Int(word_int))
        } else if (word.chars().nth(0).unwrap() == '"') && (word.chars().last().unwrap() == '"') {
            let without_quotes = &word[1..word.len() - 1];
            Token::Value(Value::String(without_quotes.to_string()))
        } else if word.chars().nth(0).unwrap() == '\'' {
            let without_quote = &word[1..];
            Token::Value(Value::Ident(without_quote.to_string()))
        } else if word == "t" {
            Token::Value(Value::True)
        } else {
            Token::Value(Value::Symbol(word.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum IntermediateToken {
    LParen(i32),
    Value(Value),
    RParen(i32),
    EOF,
    Fn,
    AstNode(AstNode),
}

#[derive(Clone)]
pub enum Function {
    Native(Rc<dyn Fn(AstNode) -> Result<AstNode, CIEvalError>>),
    User {
        varname: String,
        body: Box<AstNode>,
    },
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Native(_) => write!(f, "<native fn>"),
            Function::User { varname, body } => write!(f, "λ{} -> {:?}", varname, body),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Value(Value),
    Par {
        car: Box<AstNode>,
        cdr: Box<AstNode>
    },
    Function(Function)
}

// this is necessary for std::mem::take to work
impl Default for AstNode {
    fn default() -> Self {
        AstNode::Value(Value::Nil)
    }
}

