use core::mem::take;

use crate::{
    ast::{AstNode, IntermediateToken, Value},
    parser_types::{CIParserError, Parser},
};

pub struct TokenStream<I: Iterator<Item = IntermediateToken>> {
    iter: I
}

impl<I: Iterator<Item = IntermediateToken>> TokenStream<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }

    pub fn next(&mut self) -> Option<IntermediateToken> {
        self.iter.next()
    }

    pub fn collect_until_terminator<F>(&mut self, mut terminator: F) -> Result<Vec<AstNode>, CIParserError>
    where
        F: FnMut(&I::Item) -> bool
    {
        let mut items = Vec::new();

        while let Some(tok) = self.next() {
            if terminator(&tok) { break; }
            items.push(parse_token(tok, self)?)
        }

        Ok(items)
    }
}

fn parse_paren_nodes(mut items: Vec<AstNode>) -> Result<AstNode, CIParserError> {
    match items.len() {
        0 => Ok(AstNode::Value(Value::Nil)),
        1 => Ok(take(&mut items[0])),
        2 => Ok(AstNode::Par {
            car: Box::new(take(&mut items[0])),
            cdr: Box::new(take(&mut items[1]))
        }),
        3 if matches!(&items[0], AstNode::Value(Value::Symbol(s)) if s == "fn") => {
            let arg_ident = match take(&mut items[1]) {
                AstNode::Value(Value::Ident(name)) => name,
                a => return Err(CIParserError::UnexpectedToken(Box::new(IntermediateToken::AstNode(a)))),
            };

            Ok(AstNode::Lambda {
                varname: arg_ident,
                body: Box::new(take(&mut items[2]))
            })
        }
        _n => {
            let mut iter = items.into_iter();
            if let Some(first) = iter.next() {
                Ok(iter.fold(first, |func, arg| AstNode::Par {
                    car: Box::new(func),
                    cdr: Box::new(arg)
                }))
                
            } else {
                // Empty Node
                Ok(AstNode::Value(Value::Nil))
            }
        }
        // _ => Err(CIParserError::NodeFull(items))
    }
}

fn parse_paren(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>,
    level: i32,
) -> Result<AstNode, CIParserError> {
    let items = stream.collect_until_terminator(|tok| matches!(tok, IntermediateToken::RParen(l) if l == &level))?;
    parse_paren_nodes(items)
}

fn parse_virtual_paren(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>
) -> Result<AstNode, CIParserError> {
    let items = stream.collect_until_terminator(|tok| matches!(tok, IntermediateToken::EOF))?;
    parse_paren_nodes(items)
}

fn parse_infix_nodes(mut nodes: Vec<AstNode>) -> Result<AstNode, CIParserError> {
    match nodes.len() {
        0 => Ok(AstNode::Value(Value::Nil)),
        1 => Ok(take(&mut nodes[0])),
        2 => Ok(AstNode::Par {
           car: Box::new(take(&mut nodes[0])),
           cdr: Box::new(take(&mut nodes[1])) 
        }),
        3 => Ok(AstNode::Par {
            car: Box::new(AstNode::Par {
                car: Box::new(take(&mut nodes[1])),
                cdr: Box::new(take(&mut nodes[2])),
            }),
            cdr: Box::new(take(&mut nodes[0])),
        }),
        5 => Ok(AstNode::Par {
            // {f +/ g -/ h} => ((-/ h) ((+/ g) f))
            car: Box::new(AstNode::Par {
                car: Box::new(take(&mut nodes[3])),
                cdr: Box::new(take(&mut nodes[4]))
            }),
            cdr: Box::new(AstNode::Par {
                car: Box::new(AstNode::Par {
                    car: Box::new(take(&mut nodes[1])),
                    cdr: Box::new(take(&mut nodes[2]))
                }),
                cdr: Box::new(take(&mut nodes[0]))
            })
        }),
        _ => Err(CIParserError::NodeFull(nodes)),
    }
}

fn parse_infix(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>,
    level: i32,
) -> Result<AstNode, CIParserError> {
    let nodes = stream.collect_until_terminator(|tok| {
        matches!(tok, IntermediateToken::RCurly(l) if *l == level)
    })?;
    parse_infix_nodes(nodes)
}

fn parse_virtual_infix(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>
) -> Result<AstNode, CIParserError> {
    let nodes = stream.collect_until_terminator(|tok| {
        matches!(tok, IntermediateToken::EOF)
    })?;
    parse_infix_nodes(nodes)
}

fn parse_list(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>,
    level: i32,
) -> Result<AstNode, CIParserError> {
    let items = stream.collect_until_terminator(|tok| matches!(tok, IntermediateToken::RBracket(l) if l == &level))?;

    let mut result = AstNode::Value(Value::Nil);

    for item in items.into_iter().rev() {
        result = AstNode::Par {
            car: Box::new(AstNode::Par {
                car: Box::new(AstNode::Value(Value::Symbol("cons".to_string()))),
                cdr: Box::new(result),
            }),
            cdr: Box::new(item),
        };
    }

    Ok(result)
}

fn parse_token(
    token: IntermediateToken,
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>,
) -> Result<AstNode, CIParserError> {
    match token {
        IntermediateToken::Value(v) => Ok(AstNode::Value(v)),
        IntermediateToken::AstNode(n) => Ok(n),
        IntermediateToken::LParen(level) => parse_paren(stream, level),
        IntermediateToken::LCurly(level) => parse_infix(stream, level),
        IntermediateToken::LBracket(level) => parse_list(stream, level),
        other => Err(CIParserError::UnexpectedToken(Box::new(other))),
    }
}

fn ensure_stream_ended<I: Iterator<Item = IntermediateToken>>(stream: &mut TokenStream<I>) -> Result<(), CIParserError> {
    match stream.next() {
        None | Some(IntermediateToken::EOF) => Ok(()),
        Some(extra) => Err(CIParserError::UnexpectedToken(Box::new(extra))),
    }
}

#[derive(Default)]
pub struct CINewReplParser {
    cfg: ParserConfig
}

#[derive(Default, Clone)]
#[cfg_attr(feature = "clap_parser_mode", derive(clap::ValueEnum))]
pub enum ParserMode {
    #[default]
    Normal,
    VirtualInfix,
    VirtualParen
}

#[derive(Default, Clone)]
pub struct ParserConfig {
    pub parser_mode: ParserMode,
}

impl CINewReplParser {
    pub fn new(cfg: ParserConfig) -> Self {
        Self { cfg: cfg }
    }
}

impl Parser for CINewReplParser {
    type Input = Vec<IntermediateToken>;
    type Output = AstNode;

    fn parse(&self, tokens: Vec<IntermediateToken>) -> Result<AstNode, CIParserError> {
        let mut stream = TokenStream::new(tokens.into_iter());

        match self.cfg.parser_mode {
            ParserMode::Normal => (),
            ParserMode::VirtualInfix => {
                let result = parse_virtual_infix(&mut stream)?;
                ensure_stream_ended(&mut stream)?;
                return Ok(result);
            },
            ParserMode::VirtualParen => {
                let result = parse_virtual_paren(&mut stream)?;
                ensure_stream_ended(&mut stream)?;
                return Ok(result);
            }
        }

        match stream.next() {
            Some(IntermediateToken::LParen(level)) => {
                let result = parse_paren(&mut stream, level)?;
                ensure_stream_ended(&mut stream)?;
                Ok(result)
            }
            Some(IntermediateToken::LBracket(level)) => {
                let result = parse_list(&mut stream, level)?;
                ensure_stream_ended(&mut stream)?;
                Ok(result)
            }
            Some(IntermediateToken::LCurly(level)) => {
                let result = parse_infix(&mut stream, level)?;
                ensure_stream_ended(&mut stream)?;
                Ok(result)
            }
            Some(tok) => Err(CIParserError::UnexpectedToken(Box::new(tok))),
            None => Err(CIParserError::UnexpectedToken(Box::new(IntermediateToken::EOF))),
        }
    }
}

#[derive(Default)]
pub struct CINewFileParser {}

impl Parser for CINewFileParser {
    type Input = Vec<IntermediateToken>;
    type Output = Vec<AstNode>;

    fn parse(&self, tokens: Vec<IntermediateToken>) -> Result<Vec<AstNode>, CIParserError> {
        let mut stream = TokenStream::new(tokens.into_iter());
        let mut forms = Vec::new();

        while let Some(tok) = stream.next() {
            match tok {
                IntermediateToken::LParen(level) => {
                    let node = parse_paren(&mut stream, level)?;
                    forms.push(node);
                }
                IntermediateToken::LBracket(level) => {
                    let node = parse_list(&mut stream, level)?;
                    forms.push(node);
                }
                IntermediateToken::LCurly(level) => {
                    let node = parse_infix(&mut stream, level)?;
                    forms.push(node);
                }
                IntermediateToken::EOF => break,
                unexpected => {
                    return Err(CIParserError::UnexpectedToken(Box::new(unexpected)));
                }
            }
        }

        Ok(forms)
    }
}
