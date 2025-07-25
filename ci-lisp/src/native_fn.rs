#[macro_export]
macro_rules! native_fn {
    (($($arg_pat:pat),*), $body:block) => {
        $crate::ast::AstNode::Function($crate::ast::Function::Native(std::rc::Rc::new(
            native_fn!(@curried [$($arg_pat),*] => $body)
        )))
    };
    
    (@curried [] => $body:block) => {
        move |arg| {
            let _: () = arg;
            $body
        }
    };
    
    (@curried [$arg_pat:pat] => $body:block) => {
        move |arg| {
            match arg.clone() {
                $arg_pat => $body,
                other => Err($crate::parsers::CIEvalError::UnexpectedValue(Box::new(other.clone()))),
            }
        }
    };
    
    // The one-line fix to your original code:
    (@curried [$first_pat:pat, $($rest_pats:pat),+] => $body:block) => {
        move |arg| {
            match arg.clone() { // <- Add .clone() here
                $first_pat => Ok($crate::ast::AstNode::Function($crate::ast::Function::Native(std::rc::Rc::new(
                    native_fn!(@curried [$($rest_pats),*] => $body)
                )))),
                other => Err($crate::parsers::CIEvalError::UnexpectedValue(Box::new(other.clone()))),
            }
        }
    };
}
