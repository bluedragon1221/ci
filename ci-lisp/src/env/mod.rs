use im::HashMap;
use crate::{ast::AstNode, parser_types::{CIParserError, Parser}};

pub mod math;
pub mod prelude;

// Environment is cheap to clone thanks to im::HashMap;
#[derive(Clone, Default, Debug)]
pub struct Environment {
    bindings: HashMap<String, AstNode>
}

impl Environment {
    pub fn new(bindings: HashMap<String, AstNode>) -> Environment {
        Self {bindings}
    }

    // now this returns a new one instead of modifying the old one  
    pub fn insert(&self, cmd: &str, node: AstNode) -> Environment {
        Self::new(self.bindings.update(cmd.to_string(), node))
    }

    pub fn get(&self, key: &str) -> Option<&AstNode> {
        self.bindings.get(key)
    }
}

pub struct WrapWithEnv<P: Parser> {
    p: P,
    env: Environment
}

impl<P: Parser> WrapWithEnv<P> {
    pub fn new(p: P, env: Environment) -> Self {
        Self { p, env }
    }
}

impl<P: Parser> Parser for WrapWithEnv<P> {
    type Input = P::Input;
    type Output = (P::Output, Environment);

    fn parse(&self, tokens: Self::Input) -> Result<Self::Output, CIParserError> {
        Ok((self.p.parse(tokens)?, self.env.clone()))
    }
}

impl<P: Parser + Default> Default for WrapWithEnv<P> {
    fn default() -> Self {
        Self {
            p: P::default(),
            env: Environment::default()
        }
    }
}
