pub type Ident = String;

// ((def line) (fn x ((+ x) 2)))
// what happens when (line 3)?
//   - create new environment (clone of parent envionment)
//   - ((def x) 3)
//   - do stuff?
// #[derive(Clone, Debug)]
// pub struct Function {
//     argname: Ident,
//     code: Box<Atom>
// }

#[derive(Clone, Debug)]
pub enum Value {
    Int(i32),
    String(String),
    Ident(Ident),
}

#[derive(Clone, Debug)]
pub enum AstNode {
    Value(Value),
    Par {
        car: Box<AstNode>,
        cdr: Box<AstNode>
    }
}


