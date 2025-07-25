use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

use crate::{ast::{AstNode, Function, Value}, parser_types::{CIParserError, Parser}, parsers::CIOnlyParsing};
use crate::native_fn;

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

pub struct CIEvaluator {
    env: Rc<RefCell<HashMap<String, AstNode>>>,
}

impl Default for CIEvaluator {
    fn default() -> Self {
        let env = Rc::new(RefCell::new(HashMap::new()));

        let mut initial_bindings = env.borrow_mut();

        initial_bindings.insert("inc".to_string(), native_fn!(
            (AstNode::Value(Value::Int(a))), {
                Ok(AstNode::Value(Value::Int(a + 1)))
            }
        ));

        initial_bindings.insert("eq".to_string(), native_fn!(
            (AstNode::Value(a), AstNode::Value(b)), {
                if a == b {
                   Ok(AstNode::Value(Value::True))
                } else {
                   Ok(AstNode::Value(Value::Nil))
                }
            }
        ));

        initial_bindings.insert(
            "if".to_string(),
            AstNode::Function(Function::Native(Rc::new(
                move |cond_node: AstNode| {
                    match cond_node {
                        AstNode::Value(Value::Nil) => {
                            Ok(AstNode::Function(Function::Native(Rc::new(
                                move |_on_true_node: AstNode| {
                                    Ok(AstNode::Function(Function::Native(Rc::new(
                                        move |on_false_node: AstNode| {
                                            Ok(on_false_node.clone())
                                        },
                                    ))))
                                },
                            ))))
                        }
                        _ => {
                            Ok(AstNode::Function(Function::Native(Rc::new(
                                move |on_true_node: AstNode| {
                                    Ok(AstNode::Function(Function::Native(Rc::new(
                                        move |_on_false_node: AstNode| {
                                            Ok(on_true_node.clone())
                                        },
                                    ))))
                                },
                            ))))
                        }
                    }
                },
            ))),
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
                                    Ok(AstNode::Value(Value::Nil))
                                }
                                other => Err(CIEvalError::UnexpectedValue(Box::new(other))),
                            }
                        }
                    }))))
                }
            }))),
        );

        initial_bindings.insert(
            "include".to_string(),
            AstNode::Function(Function::Native(Rc::new({
                let env = Rc::clone(&env);
                move |arg: AstNode| {
                    // Extract filename string from arg
                    let filename = match arg {
                        AstNode::Value(Value::Ident(ref s)) | AstNode::Value(Value::Symbol(ref s)) => s.clone(),
                        AstNode::Value(Value::String(ref s)) => s.clone(),
                        other => return Err(CIEvalError::UnexpectedValue(Box::new(other))),
                    };

                    // Read the file
                    let source = fs::read_to_string(&filename)
                        .map_err(|_| CIEvalError::NoSuchFile(filename.clone()))?;

                    // Parse the source into AST nodes
                    let parser = CIOnlyParsing::default();
                    let parsed_nodes = match parser.parse(source.chars().collect()) {
                        Ok(a) => a,
                        Err(e) => return Err(CIEvalError::FileParseError(Box::new(e)))
                    };

                    // Evaluate each node in the current env
                    let local_evaluator = CIEvaluator { env: Rc::clone(&env) };
                    for node in parsed_nodes {
                        local_evaluator.eval_node(&node)?;
                    }

                    Ok(AstNode::Value(Value::Nil))
                }
            }))),
        );

        drop(initial_bindings); // unlock mutable borrow

        Self { env }
    }
}


impl CIEvaluator {
    pub fn new(custom_env: Rc<RefCell<HashMap<String, AstNode>>>) -> Self {
        // Start with the default environment bindings
        let default_evaluator = CIEvaluator::default();
        let mut merged_env = default_evaluator.env.borrow().clone();

        // Extend/override with custom_env
        merged_env.extend(custom_env.borrow().clone());

        // Wrap in Rc<RefCell> and return new evaluator
        Self {
            env: Rc::new(RefCell::new(merged_env)),
        }
    }
    
    fn eval_node(&self, node: &AstNode) -> Result<AstNode, CIEvalError> {
        match node {
            AstNode::Par { car, cdr } => {
                let func = self.eval_node(car)?;
                let arg = self.eval_node(cdr)?;

                match func {
                    AstNode::Function(Function::Native(f)) => f(arg).map_err(Into::into),

                    AstNode::Function(Function::User { varname, body, env }) => {
                        let mut new_env = (*env.borrow()).clone();
                        new_env.insert(varname.clone(), arg);
                        let new_env_rc = Rc::new(RefCell::new(new_env));
                        let subeval = CIEvaluator::new(new_env_rc);
                        subeval.eval_node(&body)
                    }

                    other => Err(CIEvalError::NonCallable(Box::new(other))),
                }
            }

            AstNode::Lambda { varname, body } => {
                Ok(AstNode::Function(Function::User {
                    varname: varname.clone(),
                    body: Box::new(*body.clone()),
                    env: Rc::clone(&self.env),
                }))
            },

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
