use ci_lisp::{ast::{Token, Value}, parser_types::Parser, parsers::CIStreamingLexer};
use egui::Event;

#[derive(Default)]
pub struct LispEvalBlock {
    is_selected: bool,
    
    input_text: String,
    cursor_pos: usize,

    output_text: Option<Box<dyn std::fmt::Display>> // later we could add different types of output, such as graphs or tables
}

impl LispEvalBlock {
    pub fn set_selected(&mut self, b: bool) {
        self.is_selected = b
    }
    pub fn set_input_text(&mut self, t: String) {
        self.input_text = t
    }
    
    fn insert_char(&mut self, ch: char) {
        match ch {
            '(' => { self.input_text.insert(self.cursor_pos, '('); self.input_text.insert(self.cursor_pos + 1, ')'); self.cursor_pos += 1; }
            '[' => { self.input_text.insert(self.cursor_pos, '['); self.input_text.insert(self.cursor_pos + 1, ']'); self.cursor_pos += 1; }
            '{' => { self.input_text.insert(self.cursor_pos, '{'); self.input_text.insert(self.cursor_pos + 1, '}'); self.cursor_pos += 1; }
            '"' => { self.input_text.insert(self.cursor_pos, '"'); self.input_text.insert(self.cursor_pos + 1, '"'); self.cursor_pos += 1; }
            ')' | ']' | '}' => {
                if self.cursor_pos < self.input_text.len() && self.input_text.chars().nth(self.cursor_pos) == Some(ch) {
                    self.cursor_pos += 1;
                } else {
                    self.input_text.insert(self.cursor_pos, ch);
                    self.cursor_pos += 1;
                }
            }
            _ => {
                self.input_text.insert(self.cursor_pos, ch);
                self.cursor_pos += 1;
            }
        }
    }

    fn handle_backspace(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }
    
        // Get the char before and after cursor (if they exist)
        let before = self.input_text.chars().nth(self.cursor_pos.saturating_sub(1));
        let after = self.input_text.chars().nth(self.cursor_pos);

        // Check for bracket pairs like (|), {|}, [|]
        let is_matching_pair = matches!(
            (before, after),
            (Some('('), Some(')'))
                | (Some('{'), Some('}'))
                | (Some('['), Some(']'))
                | (Some('"'), Some('"'))
        );

        if is_matching_pair {
            // Remove the char after cursor first (so indices stay valid)
            self.input_text.remove(self.cursor_pos);
            // Remove the char before cursor
            self.input_text.remove(self.cursor_pos - 1);
            // Move cursor back one position
            self.cursor_pos -= 1;
        } else {
            // Normal backspace
            self.input_text.remove(self.cursor_pos - 1);
            self.cursor_pos -= 1;
        }
    }

    pub fn handle_input(&mut self, event: Event) {
        match event {
            egui::Event::Key {key: egui::Key::Backspace, pressed: true, ..} => self.handle_backspace(),
            egui::Event::Key {key: egui::Key::ArrowLeft, pressed: true, ..} => if self.cursor_pos > 0 {self.cursor_pos -= 1},
            egui::Event::Key {key: egui::Key::ArrowRight, pressed: true, ..} => if self.cursor_pos < self.input_text.len() { self.cursor_pos += 1},

            egui::Event::Key {key: egui::Key::L, pressed: true, modifiers, ..} if modifiers.ctrl => {
                self.input_text.clear();
                self.cursor_pos = 0;
            }
            
            egui::Event::Text(s) => {
                for ch in s.chars() {
                    self.insert_char(ch);
                }
            }
            _ => {}
        }
    }

    fn token_color(token: &Token) -> egui::Color32 {
        match &token {
            Token::Value(Value::Int(_)) => egui::Color32::from_rgb(200, 150, 255),   // purple
            Token::Value(Value::String(_)) => egui::Color32::from_rgb(255, 200, 100),// orange
            Token::Value(Value::Symbol(_)) => egui::Color32::from_rgb(100, 200, 255),// blue
            Token::Value(Value::Ident(_)) => egui::Color32::from_rgb(150, 255, 150), // green
            Token::Value(Value::True) | Token::Value(Value::Nil) => egui::Color32::LIGHT_BLUE,
            _ => egui::Color32::WHITE,
        }
    }

    // yeah, this logic is painful
    pub fn token_str(i: usize, tokens: &Vec<Token>) -> String {
        let opener = |t: &Token| matches!(t, Token::LParen | Token::LCurly | Token::LBracket);
        let closer = |t: &Token| matches!(t, Token::RParen | Token::RCurly | Token::RBracket);
    
        let current = &tokens[i];
        let after = tokens.get(i + 1);
    
        let needs_space_after = match (current, after) {
            (_, None) => false,  // End of line
            (cur, Some(_)) if opener(cur) => false,   // (a
            (_, Some(next)) if closer(next) => false, // a)
            (cur, Some(_)) if closer(cur) => true,    // )a
            (_, Some(_)) => true,  // a
        };
    
        let mut result = current.to_string();
        if needs_space_after {
            result.push(' ');
        }
    
        result
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let font_id = egui::FontId::monospace(20.0);

        // Measure space for background rect
        let available_width = ui.available_width();
        let row_height = font_id.size + 8.0; // input line + padding

        let bg_color = egui::Color32::from_rgb(60, 60, 60); // lighter gray
        let rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(available_width, row_height));
        ui.painter().rect_filled(rect, 4.0, bg_color);

        let mut pos_x = ui.cursor().min.x;

        // Tokenize
        let tokens = CIStreamingLexer::default()
            .parse(self.input_text.clone())
            .unwrap();


        let mut caret_x = pos_x;
        let mut caret_set = false;
        let mut char_index = 0;

        for (i, token) in tokens.iter().enumerate() {
            let token_str = Self::token_str(i, &tokens);

            let color = Self::token_color(&token);

            let galley = ui.painter().layout_no_wrap(token_str.clone(), font_id.clone(), color);
            ui.painter().galley(egui::pos2(pos_x, ui.cursor().min.y), galley.clone(), color);

            if !caret_set && self.cursor_pos <= char_index + token_str.len() {
                let chars_before = self.cursor_pos - char_index;
                let width = ui.painter()
                    .layout_no_wrap(token_str.chars().take(chars_before).collect::<String>(), font_id.clone(), color)
                    .size()
                    .x;
                caret_x = pos_x + width;
                caret_set = true;
            }

            pos_x += galley.size().x;
            char_index += token_str.len();
        }

        if !caret_set {
            caret_x = pos_x;
        }

        // Caret only if selected
        if self.is_selected {
            let caret_rect = egui::Rect::from_min_size(
                egui::pos2(caret_x, ui.cursor().min.y),
                egui::vec2(1.0, font_id.size),
            );
            ui.painter().rect_filled(caret_rect, 0.0, egui::Color32::WHITE);
        }

        // Output text
        if let Some(output) = &self.output_text {
            let output_font = egui::FontId::monospace(16.0);
            let output_color = egui::Color32::from_gray(180);

            let output_galley = ui.painter().layout_no_wrap(output.to_string(), output_font, output_color);
            let output_pos = egui::pos2(ui.cursor().min.x, ui.cursor().min.y + font_id.size + 4.0);
            ui.painter().galley(output_pos, output_galley.clone(), output_color);

            ui.add_space(font_id.size + output_galley.size().y + 8.0);
        } else {
            ui.add_space(font_id.size + 8.0);
        }
    }

    pub fn eval_block<P, T>(&mut self, evaluator: &mut P)
    where
        T: std::fmt::Display + 'static + Clone,
        P: Parser<Input = String, Output = T>,
    {
        let res = evaluator.parse(self.input_text.clone());
        self.output_text = match res {
            Ok(s) => {
                Some(Box::new(s[0].clone()))
            }
            Err(e) => {
                Some(Box::new(format!("{}", e)))
            }
        };
    }
}
