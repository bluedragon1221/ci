use std::cell::RefCell;

use crate::{ast::{AstNode, Function, Value}, env::Environment, parser_types::{CIParserError, Parser}};

#[derive(Debug, thiserror::Error)]
pub enum CIEvalError {
    #[error("Unknown symbol: {0}")]
    UnknownSymbol(String),

    #[error("Non-callable value: {0:?}")]
    NonCallable(Box<AstNode>),

    #[error("Unexpected Value: {0:?}")]
    UnexpectedValue(Box<AstNode>),

    #[error("Application form is invalid")]
    InvalidApplication,

    #[error("File does not exist: {0}")]
    NoSuchFile(String),

    #[error("Error while parsing file: {0}")]
    FileParseError(#[from] Box<CIParserError>)
}

pub struct CIFileEvaluator {
    env: RefCell<Environment>
}

impl CIFileEvaluator {
    pub fn new(env: Environment) -> Self {
        Self { env: RefCell::new(env) }
    }

    pub fn take_env(self) -> Environment {
        self.env.take()
    }

    pub fn eval_node(&self, node: &AstNode, env: Environment) -> Result<(AstNode, Environment), CIEvalError> {
        match node {
            AstNode::Par { car, cdr } => {
                let func = self.eval_node(car, env.clone())?.0;
                let arg = self.eval_node(cdr, env.clone())?.0;

                match func {
                    AstNode::Function(Function::Native(f)) => Ok((f(arg)?, env)),
                    AstNode::Function(Function::NativeMutEnv(f)) => Ok(f(arg, env)?),

                    AstNode::Function(Function::User { varname, body, env: func_env }) => {
                        let (res, _) = self.eval_node(&body, func_env.insert(&varname, arg))?;
                        Ok((res, env))
                    }

                    other => Err(CIEvalError::NonCallable(Box::new(other))),
                }
            }

            AstNode::Lambda { varname, body } => {
                Ok((AstNode::Function(Function::User {
                    varname: varname.clone(),
                    body: Box::new(*body.clone()),
                    env: env.clone(),
                }), env))
            },

            AstNode::Value(Value::Symbol(s)) => {
                let val = env.get(s)
                    .ok_or(CIEvalError::UnknownSymbol(s.clone()))?;

                self.eval_node(val, env.clone())
            }

            _ => Ok((node.clone(), env)),
        }
    }
}

impl Parser for CIFileEvaluator {
    type Input = Vec<AstNode>;
    type Output = Vec<AstNode>;

    fn parse(&self, ast: Vec<AstNode>) -> Result<Vec<AstNode>, CIParserError> {
        let mut nodes = Vec::new();

        for i in ast.iter() {
            let (node, node_env) = self.eval_node(i, self.env.borrow().clone())?;
            nodes.push(node);
            *self.env.borrow_mut() = node_env;
        }

        Ok(nodes)
    }
}
