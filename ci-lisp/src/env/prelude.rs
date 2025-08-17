use std::rc::Rc;

use crate::{ast::{AstNode, Function, Value}, env::Environment, native_fn, parsers::{CIEvalError, CIFileEvaluator}};

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
                                        Ok(on_false_node)
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
                    Ok(AstNode::Value(Value::String(format!("{s1}{s2}")))) // Format as display because we don't want strings to have quotes
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

    let env = env.insert("doc", native_fn!(
        (AstNode::Value(Value::String(doc)), AstNode::Function(Function::User {varname, body, doc: _, env})), {
            Ok(AstNode::Function(Function::User {
                varname, body, doc: Some(doc.to_string()), env
            }))
        }
    ));

    let env = env.insert("help", AstNode::Function(Function::NativeMutEnv(Rc::new(|arg: AstNode, env: Environment| {
        arg.help(env.clone())?;
        Ok((AstNode::Value(Value::Nil), env))
    }))));

    let env = env.insert(
        "include",
        AstNode::Function(Function::NativeMutEnv(Rc::new(|arg: AstNode, env: Environment| {
            let filename = match arg {
                AstNode::Value(Value::String(s)) => s,
                other => return Err(CIEvalError::UnexpectedValue(Box::new(other)))
            };

            let eval = CIFileEvaluator::new(env);
            eval.load_file(filename)?;
            Ok((AstNode::Value(Value::Nil), eval.take_env()))
        })))
    );

    env
}
