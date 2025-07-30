use crate::{ast::{AstNode, Value}, env::Environment, native_fn};

pub fn math_environment(env: Environment) -> Environment {
    let env = env.insert("inc", native_fn!(
        (AstNode::Value(Value::Int(a))), {
            Ok(AstNode::Value(Value::Int(a + 1)))
        }
    ));
    let env = env.insert("dec", native_fn!(
        (AstNode::Value(Value::Int(a))), {
            Ok(AstNode::Value(Value::Int(a - 1)))
        }
    ));

    let env = env.insert("eq", native_fn!(
        (AstNode::Value(a), AstNode::Value(b)), {
            if a == b {
               Ok(AstNode::Value(Value::True))
            } else {
               Ok(AstNode::Value(Value::Nil))
            }
        }
    ));

    let env = env.insert("lt", native_fn!(
        (AstNode::Value(Value::Int(b)), AstNode::Value(Value::Int(a))), {
            if a < b {
                Ok(AstNode::Value(Value::True))
            } else {
                Ok(AstNode::Value(Value::Nil))
            }
        }
    ));

    let env = env.insert("add", native_fn!(
        (AstNode::Value(Value::Int(a)), AstNode::Value(Value::Int(b))), {
            Ok(AstNode::Value(Value::Int(a + b)))
        }
    ));
    let env = env.insert("mul", native_fn!(
        (AstNode::Value(Value::Int(a)), AstNode::Value(Value::Int(b))), {
            Ok(AstNode::Value(Value::Int(a * b)))
        }
    ));
    let env = env.insert("sub", native_fn!(
        (AstNode::Value(Value::Int(b)), AstNode::Value(Value::Int(a))), {
            Ok(AstNode::Value(Value::Int(a - b)))
        }
    ));

    env
}
