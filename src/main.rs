pub mod core;

use std::env;
use std::time::Instant;


fn main() {
    let start = Instant::now();
    let args = env::args().collect::<Vec<String>>();
    let lexed = core::lexer::lexer(args[1].clone());
    let out_file = core::codegen::asm_x86_64(lexed, &args[1].clone());
    core::codegen::assemble(out_file);
    println!("[INFO]: Compilation took {} seconds", start.elapsed().as_secs_f64());
}



