use crate::{
    ast::{AstNode, IntermediateToken, Value},
    parser_types::{CIParserError, Parser},
};

pub struct TokenStream<I: Iterator<Item = IntermediateToken>> {
    iter: I,
}

impl<I: Iterator<Item = IntermediateToken>> TokenStream<I> {
    pub fn new(iter: I) -> Self {
        Self { iter }
    }

    pub fn next(&mut self) -> Option<IntermediateToken> {
        self.iter.next()
    }
}

fn parse_paren(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>,
    level: i32,
) -> Result<AstNode, CIParserError> {
    let mut items = Vec::new();

    while let Some(tok) = stream.next() {
        match tok {
            IntermediateToken::RParen(l) if l == level => break,
            other => items.push(parse_token(other, stream)?),
        }
    }

    match items.as_slice() {
        [] => Ok(AstNode::Value(Value::Nil)),
        [a] => Ok(a.clone()),
        [AstNode::Value(Value::Symbol(s)), arg, body] if s == "fn" => {
            let arg_ident = match arg {
                AstNode::Value(Value::Ident(name)) => name.clone(),
                _ => return Err(CIParserError::UnexpectedToken(Box::new(IntermediateToken::AstNode(arg.clone())))),
            };

            Ok(AstNode::Lambda {
                varname: arg_ident,
                body: Box::new(body.clone()),
            })
        }
        [a, b] => Ok(AstNode::Par {
            car: Box::new(a.clone()),
            cdr: Box::new(b.clone()),
        }),
        _ => Err(CIParserError::NodeFull(items)),
    }
}

fn parse_infix(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>,
    level: i32,
) -> Result<AstNode, CIParserError> {
    let mut nodes = Vec::new();

    while let Some(tok) = stream.next() {
        match tok {
            IntermediateToken::RCurly(l) if l == level => break,
            other => nodes.push(parse_token(other, stream)?),
        }
    }

    match nodes.as_slice() {
        [] => Ok(AstNode::Value(Value::Nil)),
        [a] => Ok(a.clone()),
        [a, b, c] => Ok(AstNode::Par {
            car: Box::new(AstNode::Par {
                car: Box::new(b.clone()),
                cdr: Box::new(c.clone()),
            }),
            cdr: Box::new(a.clone()),
        }),
        _ => Err(CIParserError::NodeFull(nodes)),
    }
}

fn parse_list(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>,
    level: i32,
) -> Result<AstNode, CIParserError> {
    let mut items = Vec::new();

    while let Some(tok) = stream.next() {
        match tok {
            IntermediateToken::RBracket(l) if l == level => break,
            other => items.push(parse_token(other, stream)?),
        }
    }

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

        // Add support for Church numeral prefix: #7
        IntermediateToken::Hash => {
            match stream.next() {
                Some(IntermediateToken::Value(Value::Int(n))) if n >= 0 => {
                    // Build Church numeral: succ^n zero
                    let mut node = AstNode::Value(Value::Symbol("zero".to_string()));
                    for _ in 0..n {
                        node = AstNode::Par {
                            car: Box::new(AstNode::Value(Value::Symbol("succ".to_string()))),
                            cdr: Box::new(node),
                        };
                    }
                    Ok(node)
                }
                Some(other) => Err(CIParserError::UnexpectedToken(Box::new(other))),
                None => Err(CIParserError::UnexpectedToken(Box::new(IntermediateToken::EOF))),
            }
        }
        
        IntermediateToken::AstNode(n) => Ok(n),
        IntermediateToken::LParen(level) => parse_paren(stream, level),
        IntermediateToken::LCurly(level) => parse_infix(stream, level),
        IntermediateToken::LBracket(level) => parse_list(stream, level),
        other => Err(CIParserError::UnexpectedToken(Box::new(other))),
    }
}

fn parse_virtual_infix(
    stream: &mut TokenStream<impl Iterator<Item = IntermediateToken>>,
) -> Result<AstNode, CIParserError> {
    let mut nodes = Vec::new();

    while let Some(tok) = stream.next() {
        match tok {
            IntermediateToken::EOF => break,
            other => nodes.push(parse_token(other, stream)?),
        }
    }

    match nodes.as_slice() {
        [] => Ok(AstNode::Value(Value::Nil)),
        [a] => Ok(a.clone()),
        [a, b, c] => Ok(AstNode::Par {
            car: Box::new(AstNode::Par {
                car: Box::new(b.clone()),
                cdr: Box::new(c.clone()),
            }),
            cdr: Box::new(a.clone()),
        }),
        _ => Err(CIParserError::NodeFull(nodes)),
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
    infix_repl: bool
}

impl CINewReplParser {
    pub fn new(infix_repl: bool) -> Self {
        Self { infix_repl }
    }
}

impl Parser for CINewReplParser {
    type Input = Vec<IntermediateToken>;
    type Output = AstNode;

    fn parse(&self, tokens: Vec<IntermediateToken>) -> Result<AstNode, CIParserError> {
        let mut stream = TokenStream::new(tokens.into_iter());

        if self.infix_repl {
            // { ... }
            let result = parse_virtual_infix(&mut stream)?;
            ensure_stream_ended(&mut stream)?;
            return Ok(result);
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
