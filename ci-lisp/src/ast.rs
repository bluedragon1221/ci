use std::rc::Rc;

use crate::{env::Environment, parsers::CIEvalError};

#[derive(Clone, PartialEq, Eq)]
pub enum Value {
    Int(i32),
    String(String), // "var"
    Symbol(String), // var
    Ident(String), // 'var
    True,
    Nil
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::String(i) => write!(f, "\"{}\"", i),
            Value::Symbol(i) => write!(f, "{}", i),
            Value::Ident(i) => write!(f, "'{}", i),
            Value::True => write!(f, "t"),
            Value::Nil => write!(f, "nil")
        }
    }
    
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::String(i) => write!(f, "{}", i),
            Value::Symbol(i) => write!(f, "{}", i),
            Value::Ident(i) => write!(f, "'{}", i),
            Value::True => write!(f, "t"),
            Value::Nil => write!(f, "nil")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Value(Value),
    Hash,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBracket,
    RBracket,
    EOF,
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
        } else if word == "nil" {
            Token::Value(Value::Nil)
        } else {
            Token::Value(Value::Symbol(word.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum IntermediateToken {
    LParen(i32),
    Value(Value),
    Hash,
    RParen(i32),
    LCurly(i32),
    RCurly(i32),
    LBracket(i32),
    RBracket(i32),
    AstNode(AstNode),
    EOF,
}

#[derive(Clone)]
pub enum Function {
    Native(Rc<dyn Fn(AstNode) -> Result<AstNode, CIEvalError>>),
    NativeMutEnv(Rc<dyn Fn(AstNode, Environment) -> Result<(AstNode, Environment), CIEvalError>>),
    User {
        varname: String,
        body: Box<AstNode>,
        doc: Option<String>,
        env: Environment
    },
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Native(_) => write!(f, "<native fn>"),
            Function::NativeMutEnv(_) => write!(f, "<native fn>"),
            // Function::User { varname: _, body: _, env: _ } => write!(f, "{:?}", self), // this overflows the stack when it tries to render the body of a recursive function
            Function::User { varname, body: _, doc: _, env: _ } => write!(f, "<user fn {varname}>")
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Native(_) => write!(f, "<native fn>"),
            Function::NativeMutEnv(_) => write!(f, "<native fn>"),
            // Function::User { varname, body, env: _ } => write!(f, "λ{} -> {}", varname, body),
            Function::User { varname, body: _, doc: _, env: _ } => write!(f, "<user fn {varname}>")
        }
    }
}

#[derive(Clone)]
pub enum AstNode {
    Value(Value),
    Par {
        car: Box<AstNode>,
        cdr: Box<AstNode>
    },
    Lambda {
        varname: String,
        body: Box<AstNode>,
    },
    Function(Function)
}

// this is necessary for std::mem::take to work
impl Default for AstNode {
    fn default() -> Self {
        AstNode::Value(Value::Nil)
    }
}

impl std::fmt::Debug for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstNode::Value(value) => write!(f, "{:?}", value),
            AstNode::Par { car, cdr } => write!(f, "({} {})", car, cdr),
            AstNode::Lambda { varname, body } => write!(f, "(fn '{} {})", varname, body),
            AstNode::Function(function) => write!(f, "{}", function),
        }
    }
}

impl std::fmt::Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstNode::Value(value) => write!(f, "{}", value),
            AstNode::Par { car, cdr } => write!(f, "({} {})", car, cdr),
            AstNode::Lambda { varname, body } => write!(f, "(fn '{} {})", varname, body),
            AstNode::Function(function) => write!(f, "{}", function),
        }
    }
}

impl AstNode {
    pub fn help(&self, env: Environment) -> Result<(), CIEvalError> {
        match self {
            AstNode::Value(Value::Int(i)) => {
                println!("**Type:** Int");
                println!("**Value:** {i:?}");
            }
            AstNode::Value(Value::String(s)) => {
                println!("**Type:** String");
                println!("**Value:** {s:?}");
            }
            AstNode::Value(Value::True) => println!("**Value:** t"),
            AstNode::Value(Value::Nil) =>  println!("**Value:** nil"),

            AstNode::Function(Function::User {varname, body, doc, env: _}) => {
                if let Some(desc) = doc {
                    println!("**Description**:");
                    println!("{}\n",
                        desc
                            .split_terminator('\n')
                            .map(|x| format!("> {x}"))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                }
                println!("**Definition:**");
                println!("```lisp");
                println!("(fn '{varname} {body})");
                println!("```");
            }
            AstNode::Function(Function::Native(_)) | AstNode::Function(Function::NativeMutEnv(_)) => {
                println!("Native Function");
            }

            AstNode::Value(Value::Symbol(_)) => unreachable!(), // these would have already been evaluated by now
            AstNode::Par { car: _, cdr: _ } => unreachable!(),
            AstNode::Lambda {varname: _, body: _} => unreachable!(),

            AstNode::Value(Value::Ident(i)) => {
                let val = env.get(&i)
                    .ok_or(CIEvalError::UnknownSymbol(i.clone()))?;

                match val {
                    AstNode::Function(Function::User {varname: _, body: _, doc: _, env: _}) => {
                        println!("**Function:** `{}`\n", &i);
                        val.help(env.clone())?;
                    }
                    a => a.help(env.clone())?
                }
            }
        };

        Ok(())
    }
}
