use eframe::egui::{DragValue, Response, Slider, Ui, Widget};

use crate::{
    parser::model::{DATA_BASE, STACK_BASE, TEXT_BASE},
    Machine,
};

pub struct MemoryView<'a> {
    machine: &'a mut Machine,
    view_address: &'a mut usize,
    view_endian: &'a mut bool,
}

impl<'a> MemoryView<'a> {
    pub fn new(
        machine: &'a mut Machine,
        view_address: &'a mut usize,
        view_endian: &'a mut bool,
    ) -> Self {
        Self {
            machine,
            view_address,
            view_endian,
        }
    }
}

impl<'a> Widget for MemoryView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            machine,
            view_address,
            view_endian,
        } = self;

        ui.horizontal(|ui| {
            if ui.button("TEXT").clicked() {
                *view_address = TEXT_BASE as usize;
            }
            if ui.button("DATA").clicked() {
                *view_address = DATA_BASE as usize;
            }
            if ui.button("STACK").clicked() {
                *view_address = STACK_BASE as usize;
            }
            ui.checkbox(view_endian, "View little endian");
        });

        // display memory table

        let addr = *view_address;
        let width = 8;
        let height = 16;

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("");
                for y in 0..height {
                    ui.label(format!("{:08X}", addr + (y * (width * 4))));
                }
            });
            for x in 0..width {
                ui.vertical(|ui| {
                    ui.label(format!("{:02X}", x * 4));
                    for y in 0..height {
                        let addr = addr + (y * (width * 4)) + (x * 4);
                        let value = machine.read_word(addr as u32).unwrap_or(0);
                        let bytes = value.to_le_bytes();
                        let txt = if *view_endian {
                            format!(
                                "{:02X}{:02X}{:02X}{:02X}",
                                bytes[3], bytes[2], bytes[1], bytes[0]
                            )
                        } else {
                            format!(
                                "{:02X}{:02X}{:02X}{:02X}",
                                bytes[0], bytes[1], bytes[2], bytes[3]
                            )
                        };
                        ui.label(txt);
                    }
                });
            }
        });

        ui.horizontal(|ui| {
            if ui.button("???").clicked() {
                *view_address -= 8 * 16 * 4;
            }
            if ui.button("???").clicked() {
                *view_address -= 8 * 8;
            }

            if ui.button("???").clicked() {
                *view_address += 8 * 8;
            }
            if ui.button("???").clicked() {
                *view_address += 8 * 16 * 4;
            }
        })
        .response
    }
}
