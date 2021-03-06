use eframe::egui::{Frame, Response, Ui, Widget};

use crate::{parser::opcode_name, Machine};

use super::editor::{DECODE_COLOR, EXECUTE_COLOR, FETCH_COLOR, MEMORY_COLOR, WRITEBACK_COLOR};

pub struct PipelineView<'a> {
    machine: &'a mut Machine,
}

impl<'a> PipelineView<'a> {
    pub fn new(machine: &'a mut Machine) -> Self {
        Self { machine }
    }
}

impl<'a> Widget for PipelineView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let pipeline = self.machine.pipeline();
        ui.horizontal(|ui| {
            Frame::default().fill(FETCH_COLOR).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Fetch");
                    ui.label(opcode_name(pipeline.if_id.instruction).unwrap_or(""));
                });
            });

            Frame::default().fill(DECODE_COLOR).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Decode");
                    ui.label(opcode_name(pipeline.id_ex.instruction).unwrap_or(""));
                    let rd = pipeline.id_ex.rd.name();
                    let rs = pipeline.id_ex.rs.name();
                    let rt = pipeline.id_ex.rt.name();
                    let imm = pipeline.id_ex.imm;
                    let shamt = pipeline.id_ex.shamt;
                    ui.label(format!("{rd} {rs} {rt}"));
                    ui.label(format!("imm: {imm}, shamt: {shamt}"));
                });
            });

            Frame::default().fill(EXECUTE_COLOR).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Execute");
                    ui.label(opcode_name(pipeline.ex_mem.instruction).unwrap_or(""));

                    let result = pipeline.ex_mem.alu_result;

                    if pipeline.ex_mem.write || pipeline.ex_mem.read {
                        let reg = pipeline.ex_mem.write_register.name();
                        // display memory access
                        ui.label(format!("addr: {result} reg: {reg}"));
                    } else if pipeline.ex_mem.branch {
                        let should_branch = if pipeline.ex_mem.branch_not {
                            result != 0
                        } else {
                            result == 0
                        };
                        ui.label(format!("Branch: {should_branch}"));
                    } else {
                        ui.label(format!("ALU result: {result}"));
                    }
                });
            });

            Frame::default().fill(MEMORY_COLOR).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Memory");
                    ui.label(opcode_name(pipeline.mem_wb.instruction).unwrap_or(""));

                    if pipeline.mem_wb.reg_write {
                        let reg = pipeline.mem_wb.write_register.name();
                        let data = if pipeline.mem_wb.mem_to_reg {
                            pipeline.mem_wb.mem_data
                        } else {
                            pipeline.mem_wb.alu_data
                        };
                        ui.label(format!("wrote {data} to {reg}"));
                    }
                });
            });

            Frame::default().fill(WRITEBACK_COLOR).show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label("Writeback");
                    ui.label(opcode_name(pipeline.pipe_out.instruction).unwrap_or(""));
                });
            });
        })
        .response
    }
}
