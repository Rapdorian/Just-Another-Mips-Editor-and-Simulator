use std::{ops::Range, sync::Arc};

use eframe::egui::{
    text::LayoutJob, Color32, Galley, Response, ScrollArea, TextBuffer, TextEdit, TextFormat,
    TextStyle, Ui, Widget,
};

#[derive(Default)]
pub struct Console {
    text: String,
    cursor: usize,
}

impl Console {
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
    }

    pub fn print(&mut self, text: &str) {
        self.text.push_str(text);
        self.cursor = self.text.len();
    }

    pub fn error(&mut self, text: &str) {
        self.text.push('\x07');
        self.text.push_str(text);
        self.text.push('\x1b');
        self.cursor = self.text.len() + 2;
    }

    pub fn input(&mut self) -> Option<&str> {
        // if there is a newline after cursor then return that and bump the cursor up
        let mut end = self.cursor;
        let chars = self.text.chars().skip(self.cursor);
        for c in chars {
            if c == '\n' {
                let begin = self.cursor;
                self.cursor = end;
                return Some(&self.text[begin..end]);
            } else {
                end += 1;
            }
        }
        None
    }

    pub fn view<'a>(&'a mut self) -> ConsoleView<'a> {
        ConsoleView {
            text: &mut self.text,
            cursor: self.cursor,
        }
    }
}

/// Handles displaying a console
pub struct ConsoleView<'a> {
    text: &'a mut String,
    // start of input cursor
    cursor: usize,
}

impl<'a> AsRef<str> for ConsoleView<'a> {
    fn as_ref(&self) -> &str {
        self.text
    }
}

impl<'a> TextBuffer for ConsoleView<'a> {
    fn is_mutable(&self) -> bool {
        true
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        if self.cursor > char_index {
            self.text.push_str(text);
            text.len()
        } else {
            self.text.insert_str(char_index, text);
            text.len()
        }
    }

    fn delete_char_range(&mut self, mut char_range: Range<usize>) {
        char_range.start = char_range.start.max(self.cursor);
        if char_range.start < char_range.end {
            self.text.replace_range(char_range, "");
        }
    }
}

impl<'a> Widget for ConsoleView<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(&mut self)
                    .code_editor()
                    .layouter(&mut layouter),
            )
        })
    }
}

pub fn layouter(ui: &Ui, string: &str, wrap_width: f32) -> Arc<Galley> {
    let mut layout = LayoutJob::default();
    let mut sect = String::new();
    let mut color = Color32::WHITE;
    for c in string.chars() {
        if c == '\x07' {
            layout.append(&sect, 0.0, TextFormat::simple(TextStyle::Monospace, color));
            color = Color32::RED;
            sect = String::new();
        } else if c == '\x1b' {
            layout.append(&sect, 0.0, TextFormat::simple(TextStyle::Monospace, color));
            color = Color32::WHITE;
            sect = String::new();
        } else {
            sect.push(c);
        }
    }
    layout.append(&sect, 0.0, TextFormat::simple(TextStyle::Monospace, color));
    layout.wrap_width = wrap_width;
    ui.fonts().layout_job(layout)
}
