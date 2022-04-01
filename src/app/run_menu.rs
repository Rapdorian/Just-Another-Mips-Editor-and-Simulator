use std::ops::ControlFlow;

use eframe::egui::{CtxRef, Response, Ui, Widget};

use crate::{assembler, parser::model::LabelTable, syscall::Syscall, Machine, Memory};

use super::console::Console;

/// Widget that draws and manages a running the machine
///
/// When the widget is added to the Ui it steps the simulation if it is running
pub struct RunMenu<'a> {
    machine: &'a mut Machine,
    running: &'a mut bool,
    script: &'a str,
    console: &'a mut Console,
    ctx: &'a CtxRef,
}

impl<'a> RunMenu<'a> {
    pub fn new(
        ctx: &'a CtxRef,
        machine: &'a mut Machine,
        running: &'a mut bool,
        script: &'a str,
        console: &'a mut Console,
    ) -> Self {
        Self {
            ctx,
            machine,
            running,
            script,
            console,
        }
    }

    /// Draw a run button
    /// if pressed it will assmble the current contents of the editor and enable the run flag
    fn run(&mut self, ui: &mut Ui) -> Response {
        let response = ui.button("▶");
        if response.clicked() {
            self.machine.reset();
            *self.running = true;
            self.console.clear();

            let (mem, sym) = match assembler(self.script) {
                Ok(asm) => asm,
                Err(e) => {
                    self.console.error(&format!("{e}"));
                    (Memory::default(), LabelTable::default())
                }
            };

            self.machine.flash(mem, sym);
        }
        response
    }

    /// Draw a step into button
    fn step_into(&mut self, ui: &mut Ui, should_cycle: &mut bool) -> Response {
        let response = ui.button("⬇");
        if response.clicked() {
            //self.console.error("\nWARNING: Make sure you have assembled your code since stepping does not update your code\n");
            *should_cycle = true;
        }
        response
    }

    /// Draw a step over button
    fn step_over(&mut self, ui: &mut Ui) -> Response {
        let response = ui.button("➡");
        if response.clicked() {
            println!("TODO: Step over");
        }
        response
    }

    /// Draw a step out button
    fn step_out(&mut self, ui: &mut Ui) -> Response {
        let response = ui.button("⏏");
        if response.clicked() {
            println!("TODO: Step out");
        }
        response
    }

    /// Assemble but do not run this program
    fn build(&mut self, ui: &mut Ui) -> Response {
        let response = ui.button("Build");
        if response.clicked() {
            self.machine.reset();
            self.console.clear();

            let (mem, sym) = match assembler(self.script) {
                Ok(asm) => asm,
                Err(e) => {
                    self.console.error(&format!("{e}"));
                    (Memory::default(), LabelTable::default())
                }
            };

            self.machine.flash(mem, sym);
        }
        response
    }
}

/// Draws the run menu for the given machine
///
/// Also handles stepping the machine if it is enabled
impl<'a> Widget for RunMenu<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let mut should_cycle = *self.running;
        let response = self
            .run(ui)
            .union(self.step_into(ui, &mut should_cycle))
            .union(self.step_over(ui))
            .union(self.step_out(ui))
            .union(self.build(ui));

        if should_cycle && !self.machine.pending_syscall() {
            // record anything that needs to be printed to the console from a syscall
            let mut print = String::new();

            // record if we are running so we can stop it from the Quit syscall
            let mut running = *self.running;

            if let Err(e) = self.machine.cycle() {
                self.console.error(&format!("{e:#}\n"));
                *self.running = false;
            } else {
                self.machine.handle_syscall(|syscall| match syscall {
                    Syscall::Print(out) => ControlFlow::Break(print.push_str(&out)),
                    Syscall::Error(out) => {
                        ControlFlow::Break(print.push_str(&format!("\n\x07ERROR: {out}\x1b\n")))
                    }
                    Syscall::Quit => ControlFlow::Break(running = false),
                    _ => ControlFlow::Continue(()),
                });
                self.ctx.request_repaint();

                // apply the result of syscalls
                if print.len() > 0 {
                    self.console.print(&print);
                }
                *self.running = running;
            }
        }

        response
    }
}
