use ci_lisp::{
    lexer::CISugarLexer, parser::CICoreParser, repl::{CITermRepl, Repl}
};

fn main() {
    type R = CITermRepl<
        CISugarLexer,
        CICoreParser
    >;

    R::default().r#loop()
}
