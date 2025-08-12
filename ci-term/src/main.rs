use ci_lisp::{env::{math::math_environment, prelude::prelude_environment, Environment}, parser_types::SeqParsers, parsers::{CIIntermediateTokenizer, CILexer, CINewReplParser, CIReplEvaluator}};
use ci_term::{CITermRepl, Repl};
use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of library to preload
    #[arg(short = 'i')]
    preload: Vec<String>,

    /// Treat every line as an infix {...}
    #[arg(short = 'm')]
    infix_repl: bool,

    /// Enable built-in math functions. eg. add, sub, inc, dec, etc
    #[arg(long)]
    math: bool
}

fn main() {
    let args = Args::parse();

    let mut env = Environment::default();
    env = prelude_environment(env);

    if args.math {
        env = math_environment(env);
    }

    let p = SeqParsers::new(
        SeqParsers::new(
            CILexer::default(),
            CIIntermediateTokenizer::default()
        ),
        SeqParsers::new(
            CINewReplParser::new(args.infix_repl),
            CIReplEvaluator::new(args.preload, env) 
        )
    );

    let repl = CITermRepl::new(p);
    repl.r#loop()
}
