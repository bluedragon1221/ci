use crate::{ast::{AstNode, IntermediateToken, Value}, parser_types::{CIParserError, ParserState, SingleParser}};

#[derive(Default)]
pub struct ParserStepState {
    new_tokens: Vec<IntermediateToken>,
    cur_node: Vec<AstNode>,

    highest_level: i32,
    in_highest_level: bool,

    in_fn: bool,
    cur_fn: Vec<AstNode>,

    in_infix: bool,
    cur_infix: Vec<AstNode>,

    in_list: bool,
    cur_list: Vec<AstNode>
}

impl ParserStepState {
    pub fn new(tokens: &Vec<IntermediateToken>) -> Self {
        Self {
            highest_level: Self::get_highest_level(tokens),
            ..Default::default()
        }
    }

    fn push_token(&mut self, token: IntermediateToken) {
        self.new_tokens.push(token)
    }

    fn push_cur_node(&mut self, a: AstNode) {
        self.cur_node.push(a)
    }

    fn clear_cur_node(&mut self) {
        self.cur_node = Vec::new()
    }

    fn flush_cur_node(&mut self) -> Result<(), CIParserError> {
        let token = match self.cur_node.len() {
            0 => IntermediateToken::AstNode(AstNode::Value(Value::Nil)),
            1 => IntermediateToken::AstNode(std::mem::take(&mut self.cur_node[0])),
            2 => IntermediateToken::AstNode(AstNode::Par {
                car: Box::new(std::mem::take(&mut self.cur_node[0])),
                cdr: Box::new(std::mem::take(&mut self.cur_node[1]))
            }),
            _ => return Err(CIParserError::NodeFull(self.cur_node.clone()))
        };

        self.new_tokens.push(token);
        self.clear_cur_node();

        Ok(())
    }

    fn flush_cur_fn(&mut self) -> Result<(), CIParserError> {
        if self.cur_fn.len() != 2 {
            return Err(CIParserError::NodeFull(self.cur_fn.clone()));
        }

        if let AstNode::Value(Value::Ident(ident)) = &self.cur_fn[0] {
            self.new_tokens.push(IntermediateToken::AstNode(AstNode::Lambda { varname: ident.clone(), body: Box::new(std::mem::take(&mut self.cur_fn[1])) }))
        } else {
            return Err(CIParserError::UnexpectedToken(Box::new(IntermediateToken::AstNode(std::mem::take(&mut self.cur_fn[0])))))
        }

        self.cur_fn.clear();

        Ok(())
    }

    fn flush_cur_infix(&mut self) -> Result<(), CIParserError> {
        // {1 + 2} => ((+ 2) 1)

        if self.cur_infix.len() == 0 {
            self.new_tokens.push(IntermediateToken::AstNode(AstNode::Value(Value::Nil)));
        } else if self.cur_infix.len() == 1 {
            self.new_tokens.push(IntermediateToken::AstNode(std::mem::take(&mut self.cur_infix[0])));
        } else {
            if self.cur_infix.len() != 3 {
                return Err(CIParserError::NodeFull(self.cur_infix.clone()));
            }

            self.new_tokens.push(IntermediateToken::AstNode(AstNode::Par {
                car: Box::new(AstNode::Par {
                    car: Box::new(std::mem::take(&mut self.cur_infix[1])),
                    cdr: Box::new(std::mem::take(&mut self.cur_infix[2]))
                }),
                cdr: Box::new(std::mem::take(&mut self.cur_infix[0]))
            }));
        }

        self.cur_infix.clear();

        Ok(())
    }

    fn flush_cur_list(&mut self) -> Result<(), CIParserError> {
        let mut result = AstNode::Value(Value::Nil);

        for elem in self.cur_list.iter().rev() {
            result = AstNode::Par {
                car: Box::new(AstNode::Par {
                    car: Box::new(AstNode::Value(Value::Symbol("cons".into()))),
                    cdr: Box::new(result),
                }),
                cdr: Box::new(elem.clone()),
            };
        }

        self.new_tokens.push(IntermediateToken::AstNode(result));
        self.cur_list.clear();

        Ok(())
    }

    fn get_highest_level(tokens: &Vec<IntermediateToken>) -> i32 {
        tokens.iter()
            .filter_map(|x| match x {
                IntermediateToken::LParen(n) => Some(*n),
                IntermediateToken::RParen(n) => Some(*n),
                IntermediateToken::LCurly(n) => Some(*n),
                IntermediateToken::RCurly(n) => Some(*n),
                IntermediateToken::LBracket(n) => Some(*n),
                IntermediateToken::RBracket(n) => Some(*n),
                _ => None
            })
            .max()
            .unwrap_or(0)
    }
}

impl ParserState for ParserStepState {
    type Output = Vec<IntermediateToken>;

    fn take_tokens(self) -> Self::Output {
        self.new_tokens
    }
}

#[derive(Default)]
pub struct ParserStep {}

impl SingleParser for ParserStep {
    type Input = Vec<IntermediateToken>;
    type Output = Vec<IntermediateToken>;
    type State = ParserStepState;

    fn init_state(tokens: &Self::Input) -> Self::State {
        ParserStepState::new(&tokens)
    }

    fn handle_token(token: IntermediateToken, state: &mut ParserStepState) -> Result<(), CIParserError> {
        match token {
            IntermediateToken::LParen(level) if level == state.highest_level => {
                state.in_highest_level = true;
            }
            IntermediateToken::LCurly(level) if level == state.highest_level => {
                state.in_highest_level = true;
                state.in_infix = true;
            }
            IntermediateToken::LBracket(level) if level == state.highest_level => {
                state.in_highest_level = true;
                state.in_list = true;
            }
            IntermediateToken::Fn => {
                // fn must be the first token in parens ()
                if state.in_highest_level {
                    if state.cur_node.len() != 0 || state.in_infix || state.in_list {
                        return Err(CIParserError::UnexpectedToken(Box::new(IntermediateToken::Fn)));
                    }

                    state.in_fn = true;
                } else {
                    state.push_token(IntermediateToken::Fn)
                }
            }
            IntermediateToken::Value(value) => {
                if state.in_highest_level {
                    if state.in_fn {
                        state.cur_fn.push(AstNode::Value(value));
                    } else if state.in_infix {
                        state.cur_infix.push(AstNode::Value(value));
                    } else if state.in_list {
                        state.cur_list.push(AstNode::Value(value));
                    } else {
                        state.push_cur_node(AstNode::Value(value));
                    }
                } else {
                    state.push_token(IntermediateToken::AstNode(AstNode::Value(value)));
                }
            },
            IntermediateToken::AstNode(ast_node) if state.in_highest_level => {
                if state.in_fn {
                    state.cur_fn.push(ast_node);
                } else if state.in_infix {
                    state.cur_infix.push(ast_node);
                } else if state.in_list {
                    state.cur_list.push(ast_node);
                } else {
                    state.push_cur_node(ast_node);
                }
            }
            IntermediateToken::RParen(level) if level == state.highest_level => {
                if state.in_fn {
                    state.flush_cur_fn()?;
                    state.in_fn = false;
                } else {
                    state.flush_cur_node()?;
                }
                state.in_highest_level = false;
            },
            IntermediateToken::RCurly(level) if level == state.highest_level => {
                if state.in_infix {
                    state.flush_cur_infix()?;
                    state.in_infix = false;
                }
                state.in_highest_level = false;
            }
            IntermediateToken::RBracket(level) if level == state.highest_level => {
                if state.in_list {
                    state.flush_cur_list()?;
                    state.in_list = false;
                }
                state.in_highest_level = false;
            }
            IntermediateToken::EOF => (),
            a => state.push_token(a)
        }

        Ok(())
    }
}

