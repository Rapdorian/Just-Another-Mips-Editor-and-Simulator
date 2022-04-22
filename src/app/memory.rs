use eframe::egui::{DragValue, Response, Slider, Ui, Widget};

use crate::{
    parser::model::{DATA_BASE, STACK_BASE, TEXT_BASE},
    Machine,
};

pub struct MemoryView<'a> {
    machine: &'a mut Machine,
    view_address: &'a mut usize,
}

impl<'a> MemoryView<'a> {
    pub fn new(machine: &'a mut Machine, view_address: &'a mut usize) -> Self {
        Self {
            machine,
            view_address,
        }
    }
}

impl<'a> Widget for MemoryView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            machine,
            view_address,
        } = self;
        ui.add(
            DragValue::new(view_address)
                .prefix("Address: ")
                .speed(0x1024),
        );
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
                        ui.label(format!(
                            "{:02X}{:02X}{:02X}{:02X}",
                            bytes[0], bytes[1], bytes[2], bytes[3]
                        ));
                    }
                });
            }
        })
        .response
    }
}
