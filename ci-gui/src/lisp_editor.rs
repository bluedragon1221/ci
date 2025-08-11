use ci_lisp::{parser_types::SeqParsers, parsers::{CIIntermediateTokenizer, CINewReplParser, CIReplEvaluator, CIStreamingLexer}};

use crate::LispEvalBlock;

type LispEditorParser = SeqParsers<SeqParsers<CIStreamingLexer, CIIntermediateTokenizer>, SeqParsers<CINewReplParser, CIReplEvaluator>>;

pub struct LispEditor {
    block: Vec<LispEvalBlock>,
    selected_block: usize,
    
    evaluator: LispEditorParser
}

impl LispEditor {
        pub fn new(evaluator: LispEditorParser) -> Self {
        let mut this = Self {
            block: vec![LispEvalBlock::default()],
            selected_block: 0,
            evaluator,
        };
        this.select_block(0);
        this
    }

    pub fn add_block(&mut self) {
        self.block.push(LispEvalBlock::default());
        self.select_block(self.block.len() - 1);
    }

    pub fn rm_cur_block(&mut self) {
        self.block.remove(self.selected_block);

        if self.selected_block >= self.block.len() {
            self.selected_block = self.block.len() - 1;
        }

        self.select_block(self.selected_block);
    }

    pub fn next_block(&mut self) {
        self.select_block((self.selected_block + 1) % self.block.len());
    }

    pub fn prev_block(&mut self) {
        self.select_block((self.selected_block + self.block.len() - 1) % self.block.len());
    }

    fn select_block(&mut self, index: usize) {
        self.block[self.selected_block].set_selected(false);
        self.selected_block = index;
        self.block[index].set_selected(true);
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let input = ui.input(|i| i.clone());
        for event in input.events {
            match event {
                egui::Event::Key {key: egui::Key::Enter, pressed: true, ..} => {
                    for i in self.block.iter_mut() {
                        i.eval_block(&mut self.evaluator);
                    }

                    if self.selected_block == (self.block.len() - 1) {
                        self.add_block();
                    }
                }
                egui::Event::Key {key: egui::Key::Tab, pressed: true, modifiers,  ..} if !modifiers.shift => {
                    self.next_block();
                }
                egui::Event::Key {key: egui::Key::Tab, pressed: true, modifiers, ..} if modifiers.shift => {
                    self.prev_block();
                }
                egui::Event::Key {key: egui::Key::J, pressed: true, modifiers, ..} if modifiers.ctrl => {
                    self.add_block();
                }
                egui::Event::Key {key: egui::Key::D, pressed: true, modifiers, ..} if modifiers.ctrl => {
                    self.rm_cur_block();
                }
                a => self.block[self.selected_block].handle_input(a),
            }
        }

        for i in self.block.iter_mut() {
            i.show(ui)
        }
    }
}

impl eframe::App for LispEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show(ui);
        });
    }
}

