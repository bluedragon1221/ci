use ci_lisp::{env::{math::math_environment, prelude::prelude_environment, Environment}, parser_types::SeqParsers, parsers::{CIIntermediateTokenizer, CILexer, CINewReplParser, CIReplEvaluator}};
use ci_term::{CITermRepl, Repl};
use ci_cli::Args;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let mut env = Environment::default();
    env = prelude_environment(env);

    if !args.no_math {
        env = math_environment(env);
    }

    let p = SeqParsers::new(
        SeqParsers::new(
            CILexer::default(),
            CIIntermediateTokenizer::default()
        ),
        SeqParsers::new(
            CINewReplParser::new(args.clone().into()),
            CIReplEvaluator::new(args.include, env) 
        )
    );

    let repl = CITermRepl::new(p);
    repl.r#loop()
}
