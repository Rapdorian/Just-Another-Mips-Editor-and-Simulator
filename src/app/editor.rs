use eframe::egui::{Response, ScrollArea, TextBuffer, TextEdit, Ui, Widget};

pub struct Editor<'a> {
    text: &'a mut dyn TextBuffer,
}

impl<'a> Editor<'a> {
    pub fn new(text: &'a mut dyn TextBuffer) -> Self {
        Self { text }
    }
}

impl<'a> Widget for Editor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                ui.available_size(),
                TextEdit::multiline(self.text).code_editor(),
            )
        })
    }
}
