use ci_gui::LispEditor;
use ci_lisp::{env::{math::math_environment, prelude::prelude_environment, Environment}, parser_types::SeqParsers, parsers::{CIIntermediateTokenizer, CINewReplParser, CIReplEvaluator, CIStreamingLexer}};
use ci_cli::Args;

use eframe::egui;
use clap::Parser;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    let args = Args::parse();

    let mut env = Environment::default();
    env = prelude_environment(env);
    if !args.no_math { env = math_environment(env); }
    
    let parser = SeqParsers::new(
        SeqParsers::new(
            CIStreamingLexer::default(),
            CIIntermediateTokenizer::default(),
        ),
        SeqParsers::new(
            CINewReplParser::new(args.clone().into()),
            CIReplEvaluator::new(args.include, env)
        )
    );
    
    eframe::run_native(
        "ci-gui",
        options,
        Box::new(|_| Ok(Box::new(LispEditor::new(parser)))),
    )
}
