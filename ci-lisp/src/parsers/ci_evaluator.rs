use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{ast::{AstNode, Function, Value}, parser_types::{CIParserError, Parser}};

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
}

pub struct CIEvaluator {
    env: Rc<RefCell<HashMap<String, AstNode>>>,
}

impl Default for CIEvaluator {
    fn default() -> Self {
        let env = Rc::new(RefCell::new(HashMap::new()));

        let mut initial_bindings = env.borrow_mut();

        initial_bindings.insert(
            "add".to_string(),
            AstNode::Function(Function::Native(Rc::new(|x| match x {
                AstNode::Value(Value::Int(n)) => {
                    Ok(AstNode::Function(Function::Native(Rc::new(move |x2| {
                        match x2 {
                            AstNode::Value(Value::Int(m)) => Ok(AstNode::Value(Value::Int(n + m))),
                            other => Err(CIEvalError::UnexpectedValue(Box::new(other))),
                        }
                    }))))
                }
                other => Err(CIEvalError::UnexpectedValue(Box::new(other))),
            }))),
        );

        initial_bindings.insert(
            "def".to_string(),
            AstNode::Function(Function::Native(Rc::new({
                let env = Rc::clone(&env);
                move |body: AstNode| {
                    Ok(AstNode::Function(Function::Native(Rc::new({
                        let env = Rc::clone(&env);
                        move |name: AstNode| {
                            match name {
                                AstNode::Value(Value::Ident(var)) => {
                                    env.borrow_mut().insert(var.clone(), body.clone());
                                    // Ok(AstNode::Value(Value::Ident(var)))
                                    Ok(AstNode::Value(Value::Nil))
                                }
                                other => Err(CIEvalError::UnexpectedValue(Box::new(other))),
                            }
                        }
                    }))))
                }
            }))),
        );

        drop(initial_bindings); // unlock mutable borrow

        Self { env }
    }
}


impl CIEvaluator {
    fn eval_node(&self, node: &AstNode) -> Result<AstNode, CIEvalError> {
        match node {
            AstNode::Par { car, cdr } => {
                let func = self.eval_node(car)?;
                let arg = self.eval_node(cdr)?;

                match func {
                    AstNode::Function(Function::Native(f)) => f(arg).map_err(Into::into),

                    AstNode::Function(Function::User { varname, body }) => {
                        let mut new_env = (*self.env.borrow()).clone();
                        new_env.insert(varname.clone(), arg);
                        let new_env_rc = Rc::new(RefCell::new(new_env));
                        let subeval = CIEvaluator { env: new_env_rc };
                        subeval.eval_node(&body)
                    }

                    other => Err(CIEvalError::NonCallable(Box::new(other))),
                }
            }

            AstNode::Value(Value::Symbol(s)) => {
                self.env.borrow().get(s)
                    .cloned()
                    .ok_or(CIEvalError::UnknownSymbol(s.clone()))
            }

            _ => Ok(node.clone()),
        }
    }
}

impl Parser for CIEvaluator {
    type InputNode = AstNode;
    type OutputNode = AstNode;
    
    fn parse(&self, ast: Vec<AstNode>) -> Result<Vec<AstNode>, CIParserError> {
        ast.iter().map(|n| Ok(self.eval_node(n)?)).collect()
    }
}
