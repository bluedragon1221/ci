use ci_lisp::{parsers::{CILexer, CIParser}, repl::{CITermRepl, Repl}};

fn main() {
    type R = CITermRepl<CIParser>;

    R::default().r#loop()
}
