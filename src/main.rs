#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // if let Err(e) = run() {
    //     eprintln!("ERROR: {}", e);
    // }
    let app = simulator::App::default();
    let opts = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), opts);
    let mut pc = 0;
                    mem.push(word.asm(&labels, pc));
                    pc += 4;

    println!("{:X?}", mem);
}

// fn run() -> Result<()> {
//     // parse command line args
//     let matches = App::new("mips simulator")
//         .version("0.1")
//         .author("James Pruitt <jamescpruitt@gmail.com>")
//         .arg(
//             Arg::with_name("INPUT")
//                 .help("Sets the mips image file to use")
//                 .required(true),
//         )
//         .arg(
//             Arg::with_name("single_cycle")
//                 .short("1")
//                 .long("single-cycle")
//                 .takes_value(false)
//                 .help("Tells the machine to run in single cycle mode instead of pipelined"),
//         )
//         .arg(
//             Arg::with_name("debug")
//                 .short("d")
//                 .long("debug")
//                 .takes_value(false)
//                 .help("Displays what instruction is in each stage of the pipeline"),
//         )
//         .get_matches();
//
//     // create and run image
//     let img_path = matches.value_of("INPUT").context("INPUT required")?;
//     let single_cycle = matches.is_present("single_cycle");
//     let debug = matches.is_present("debug");
//
//     // read file as string
//     let mut mem = vec![];
//
//     let script = fs::read_to_string(img_path)?;
//
//     // parse assembly
//     let lines = parser::parse_string(&script)?;
//     let labels = compute_labels(&lines);
//
//     // for each line in the parsed assembly assemble that line and add the result to a vec
//     for line in &lines {
//         match line {
//             Line::Instruction(ins) => {
//                 for word in ins {
//                     mem.push(word.asm(&labels));
//                 }
//             }
//             Line::Label(_) => {}
//         }
//     }
//
//     // create our memory object
//     let mut mem = Memory::from_word_vec(mem);
//
//     // instantiate machine
//     let mut pc = 0;
//     let mut regs = RegisterFile::default();
//
//     let mut state = PipelineState::default();
//
//     // cycle the CPU forever
//     loop {
//         if single_cycle {
//             pipeline::single_cycle(&mut pc, &mut regs, &mut mem);
//         } else {
//             state = pipeline::pipe_cycle(&mut pc, &mut regs, &mut mem, state);
//
//             if debug {
//                 // display state
//                 println!("Fetch     Decode    Execute   Memory    Writeback");
//                 println!(
//                     "{:08X}  {:08X}  {:08X}  {:08X}  {:08X}\n",
//                     state.if_id.instruction,
//                     state.id_ex.instruction,
//                     state.ex_mem.instruction,
//                     state.mem_wb.instruction,
//                     state.pipe_out.instruction,
//                 );
//             }
//         }
//         //println!("{:?}", regs);
//     }
// }
