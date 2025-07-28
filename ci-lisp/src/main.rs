use ci_lisp::{parser_types::SeqParsers, parsers::{CIIntermediateTokenizer, CILexer, CINewReplParser, CIReplEvaluator}, repl::{CITermRepl, Repl}};
use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Name of library to preload
    #[arg(short = 'i')]
    preload: Vec<String>,

    #[arg(short = 'm')]
    infix_repl: bool
}

fn main() {
    let args = Args::parse();
    
    let p = SeqParsers::new(
        SeqParsers::new(
            CILexer::default(),
            CIIntermediateTokenizer::default()
        ),
        SeqParsers::new(
            CINewReplParser::new(args.infix_repl),
            CIReplEvaluator::new(args.preload)
        )
    );

    let repl = CITermRepl::new(p);
    repl.r#loop();
}
