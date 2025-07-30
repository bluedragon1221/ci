use std::{fs, rc::Rc};

use crate::{ast::{AstNode, Function, Value}, env::Environment, parser_types::Parser, parsers::{CIEvalError, CIFileEvaluator, CIFullFileParser}};

pub fn prelude_environment(env: Environment) -> Environment {
    let env = env.insert(
        "if",
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

    let env = env.insert(
        "inspect_env",
        AstNode::Function(Function::NativeMutEnv(Rc::new(|_body: AstNode, env1: Environment| {
            println!("{env1:#?}");
            Ok((AstNode::Value(Value::Nil), env1))
        })))
    );

    let env = env.insert(
        "str_concat",
        AstNode::Function(Function::Native(Rc::new(|s2: AstNode| {
                Ok(AstNode::Function(Function::Native(Rc::new(move |s1: AstNode| {
                    Ok(AstNode::Value(Value::String(format!("{s1}{s2}"))))
                }))))
        })))
    );

    let env = env.insert(
        "log",
        AstNode::Function(Function::Native(Rc::new(|msg: AstNode| {
            Ok(AstNode::Function(Function::Native(Rc::new(move |body: AstNode| {
                println!("[log] {msg}");
                Ok(body)
            }))))
        })))
    );

    let env = env.insert(
        "def",
        AstNode::Function(Function::Native(Rc::new(|body: AstNode| {
            Ok(AstNode::Function(Function::NativeMutEnv(Rc::new(move |name: AstNode, env1: Environment| {
                match name {
                    AstNode::Value(Value::Ident(fn_name)) => {
                        Ok((AstNode::Value(Value::Nil), env1.insert(&fn_name, body.clone())))
                    },
                    other => Err(CIEvalError::UnexpectedValue(Box::new(other)))
                }
            }))))
        })))
    );

    let env = env.insert(
        "include",
        AstNode::Function(Function::NativeMutEnv(Rc::new(|arg: AstNode, env: Environment| {
            let filename = match arg {
                AstNode::Value(Value::String(s)) => s,
                other => return Err(CIEvalError::UnexpectedValue(Box::new(other)))
            };

            let source = fs::read_to_string(&filename)
                .map_err(|_| CIEvalError::NoSuchFile(filename))?;

            let parser = CIFullFileParser::default();
                let parsed_nodes = match parser.parse(source.chars().collect()) {
                    Ok(a) => a,
                    Err(e) => return Err(CIEvalError::FileParseError(Box::new(e)))
                };

                // Evaluate each node in the current env
                let evaluator = CIFileEvaluator::new(env);
                let _nodes = evaluator.parse(parsed_nodes)
                    .map_err(|e| CIEvalError::FileParseError(Box::new(e)))?;

                Ok((AstNode::Value(Value::Nil), evaluator.take_env()))
        })))
    );

    env
}
