use eframe::egui::{ComboBox, Response, TextEdit, Ui, Widget};

use crate::{parser::int, Machine, Register};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum WatchType {
    Register,
    Memory,
}

impl Default for WatchType {
    fn default() -> Self {
        Self::Register
    }
}

impl WatchType {
    /// The pretty name of this type of watch
    pub fn label(&self) -> &str {
        match self {
            WatchType::Register => "Register",
            WatchType::Memory => "Memory",
        }
    }
}

/// Represents a single watch allows its contents to be read and mutated
#[derive(Default, Clone, Copy)]
pub struct Watch {
    ty: WatchType,
    val: u32,
}

impl Watch {
    /// Read the contents of this watch
    pub fn read(&self, vm: &Machine) -> u32 {
        match self.ty {
            WatchType::Register => vm.register(self.val.into()),
            WatchType::Memory => vm.read_word(self.val),
        }
    }

    /// Write a value into the contents of this watch
    pub fn write(&self, vm: &mut Machine, val: u32) {
        match self.ty {
            WatchType::Register => *vm.register_mut(self.val.into()) = val,
            WatchType::Memory => vm.write_word(self.val, val),
        }
    }
}

/// Displays a single watch and allows it and its contents to be mutated
pub struct WatchView<'a> {
    watch: &'a mut Watch,
    vm: &'a mut Machine,
    id: usize,
}

impl<'a> WatchView<'a> {
    pub fn new(watch: &'a mut Watch, vm: &'a mut Machine, id: usize) -> Self {
        Self { watch, vm, id }
    }
}

impl<'a> Widget for WatchView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ComboBox::from_id_source(format!("{}_type", self.id))
                .selected_text(self.watch.ty.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.watch.ty, WatchType::Register, "Register");
                    ui.selectable_value(&mut self.watch.ty, WatchType::Memory, "Memory");
                });

            match self.watch.ty {
                WatchType::Register => {
                    ComboBox::from_id_source(format!("{}_reg_value", self.id))
                        .selected_text(Register::from(self.watch.val).name())
                        .show_ui(ui, |ui| {
                            for reg in 0..32 {
                                ui.selectable_value(
                                    &mut self.watch.val,
                                    reg,
                                    Register::from(reg).name(),
                                );
                            }
                        })
                        .response
                }
                WatchType::Memory => {
                    let mut val = self.watch.val.to_string();
                    let len = val.len() as f32;
                    let resp = ui.add(TextEdit::singleline(&mut val).desired_width(10.0 * len));
                    if let Ok(val) = int::<u32>(&val) {
                        self.watch.val = val.1;
                    } else if val.trim().len() == 0 {
                        self.watch.val = 0;
                    }
                    resp
                }
            };

            // handle mutating the contents of memory
            // TODO: Break this numeric text edit into a widget
            let mut contents = self.watch.read(&self.vm).to_string();
            let len = contents.len() as f32;
            ui.add(
                TextEdit::singleline(&mut contents)
                    .desired_width(10.0 * len)
                    .frame(false),
            );
            if let Ok((_, val)) = int::<u32>(&contents) {
                self.watch.write(self.vm, val);
            } else if contents.trim().len() == 0 {
                self.watch.write(self.vm, 0);
            }
        })
        .response
    }
}

/// Draws a list of watches and allows new watches to be added to the list
pub struct WatchList<'a> {
    watches: &'a mut Vec<Watch>,
    vm: &'a mut Machine,
}

impl<'a> WatchList<'a> {
    pub fn new(watches: &'a mut Vec<Watch>, vm: &'a mut Machine) -> Self {
        Self { watches, vm }
    }
}

impl<'a> Widget for WatchList<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        // If there are no watches we need to add one or this widget breaks
        if self.watches.len() == 0 {
            self.watches.push(Watch::default());
        }

        ui.vertical(|ui| {
            let mut del_list = vec![];
            for i in 0..self.watches.len() {
                ui.horizontal(|ui| {
                    ui.add(WatchView::new(&mut self.watches[i], self.vm, i));
                    if i != self.watches.len() - 1 {
                        // draw this watch with a delete button
                        // if we want to delete this watch record its index so we can delete it once we are
                        // done drawing all watches
                        if ui.button("🗑").clicked() {
                            del_list.push(i);
                        }
                    } else {
                        // draw this watch with a add button
                        // the last element is the current temporary watch so push an new watch to the list
                        // to save
                        if ui.button("+").clicked() {
                            self.watches.push(Watch::default());
                        }
                    }
                });
            }
            *self.watches = self
                .watches
                .iter()
                .enumerate()
                .filter(|(i, _)| !del_list.contains(i))
                .map(|(_, e)| *e)
                .collect();
        })
        .response
    }
}
