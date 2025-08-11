use ci_gui::LispEditor;
use ci_lisp::{env::{math::math_environment, prelude::prelude_environment, Environment}, parser_types::SeqParsers, parsers::{CIIntermediateTokenizer, CINewReplParser, CIReplEvaluator, CIStreamingLexer}};
use eframe::egui;

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


fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let args = Args::parse();

    let mut env = Environment::default();
    env = prelude_environment(env);
    if args.math { env = math_environment(env); }
    
    let parser = SeqParsers::new(
        SeqParsers::new(
            CIStreamingLexer::default(),
            CIIntermediateTokenizer::default(),
        ),
        SeqParsers::new(
            CINewReplParser::new(args.infix_repl),
            CIReplEvaluator::new(args.preload, env)
        )
    );
    
    eframe::run_native(
        "Lisp Editor",
        options,
        Box::new(|_| Ok(Box::new(LispEditor::new(parser)))),
    )
}
