use ci_lisp::parsers::ParserMode;

#[derive(clap::Parser, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of library to include. Pass multiple times for multiple libraries
    #[arg(short = 'i')]
    pub include: Vec<String>,

    /// Treat line as an infix {} or as parens ()
    #[arg(long, short = 'm', default_value = "normal")]
    pub parser_mode: ParserMode,

    /// Disable built-in math functions. eg. add, sub, inc, dec, etc
    #[arg(long)]
    pub no_math: bool
}

impl Into<ci_lisp::parsers::ParserConfig> for Args {
    fn into(self) -> ci_lisp::parsers::ParserConfig {
        ci_lisp::parsers::ParserConfig {
            parser_mode: self.parser_mode.into(),
        }
    }
}

