use std::sync::Arc;

use eframe::{
    egui::{
        text::LayoutJob, Align2, Color32, Galley, Response, ScrollArea, TextEdit, TextFormat,
        TextStyle, Ui, Widget,
    },
    epaint::FontId,
};

pub struct Editor<'a> {
    text: &'a mut String,
    pc: &'a [Option<usize>],
}

impl<'a> Editor<'a> {
    pub fn new(text: &'a mut String, pc: &'a [Option<usize>]) -> Self {
        Self { text, pc }
    }
}

impl<'a> Widget for Editor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { text, pc } = self;
        ScrollArea::vertical()
            .show(ui, |ui| {
                let resp = ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(text)
                        .code_editor()
                        .layouter(&mut |ui, s, ww| layouter(ui, s, ww, pc)),
                );

                // create line string
                let line_numbers = text
                    .lines()
                    .enumerate()
                    .fold(String::new(), |acc, (i, _)| format!("{acc}{}\n", i + 1));

                let origin = resp.rect.min;
                ui.painter().text(
                    origin,
                    Align2::LEFT_TOP,
                    &line_numbers,
                    FontId::monospace(12.0),
                    ui.style().visuals.widgets.noninteractive.fg_stroke.color,
                );
                resp
            })
            .inner
    }
}

pub const FETCH_COLOR: Color32 = Color32::from_rgb(102, 57, 49);
pub const DECODE_COLOR: Color32 = Color32::from_rgb(82, 75, 36);
pub const EXECUTE_COLOR: Color32 = Color32::from_rgb(50, 60, 57);
pub const MEMORY_COLOR: Color32 = Color32::from_rgb(63, 63, 115);
pub const WRITEBACK_COLOR: Color32 = Color32::from_rgb(69, 40, 60);

pub fn layouter<'a>(
    ui: &Ui,
    string: &str,
    wrap_width: f32,
    pc: &'a [Option<usize>],
) -> Arc<Galley> {
    let mut layout = LayoutJob::default();

    for (i, line) in string.lines().enumerate() {
        let bg = if pc[4].map(|x| x == i).unwrap_or(false) {
            WRITEBACK_COLOR
        } else if pc[3].map(|x| x == i).unwrap_or(false) {
            MEMORY_COLOR
        } else if pc[2].map(|x| x == i).unwrap_or(false) {
            EXECUTE_COLOR
        } else if pc[1].map(|x| x == i).unwrap_or(false) {
            DECODE_COLOR
        } else if pc[0].map(|x| x == i).unwrap_or(false) {
            FETCH_COLOR
        } else {
            ui.style().visuals.extreme_bg_color
        };

        let mut span = String::new();
        let mut comment = false;
        let mut idx = 0;
        let mut indent = true;
        let fg = ui.style().visuals.widgets.noninteractive.fg_stroke.color;

        let handle_token = |span: &mut String,
                            comment: &mut bool,
                            idx: &mut usize,
                            indent: &mut bool,
                            layout: &mut LayoutJob| {
            if span.starts_with("#") {
                *comment = true;
            }
            let fg = if *comment {
                Color32::from_rgb(155, 173, 183)
            } else {
                if *idx == 0 {
                    if span.starts_with(".") {
                        Color32::from_rgb(95, 205, 228)
                    } else {
                        Color32::from_rgb(217, 87, 99)
                    }
                } else {
                    if span.starts_with("$") {
                        Color32::from_rgb(106, 190, 48)
                    } else {
                        fg
                    }
                }
            };

            layout.append(
                &format!("{span}"),
                if *indent { 30.0 } else { 0.0 },
                TextFormat {
                    font_id: FontId::monospace(12.0),
                    background: bg,
                    color: fg,
                    ..TextFormat::default()
                },
            );
            if span.trim().len() > 0 {
                *idx += 1;
            }
            *indent = false;
            span.clear();
        };

        for c in line.chars() {
            // handle syntax
            if c.is_whitespace() {
                span.push(c);
                handle_token(&mut span, &mut comment, &mut idx, &mut indent, &mut layout);
            } else if c == ',' {
                handle_token(&mut span, &mut comment, &mut idx, &mut indent, &mut layout);
                span.push(',');
                handle_token(&mut span, &mut comment, &mut idx, &mut indent, &mut layout);
            } else {
                span.push(c);
            }
        }
        span.push('\n');
        handle_token(&mut span, &mut comment, &mut idx, &mut indent, &mut layout);
    }
    ui.fonts().layout_job(layout)
}
