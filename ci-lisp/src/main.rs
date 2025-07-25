use ci_lisp::{parsers::CIParser, repl::{CITermRepl, Repl}};

fn main() {
    type R = CITermRepl<CIParser>;

    R::default().r#loop()
}
