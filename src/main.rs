mod bitwise_utils;
mod vm;

use std::{env::args, process::exit, path::Path};

use vm::Lc3Vm;

fn print_usage(program_name: &str) {
    eprintln!(
        "USAGE: {program_name} LC3_PROGRAM_PATH
        LC3_PROGRAM_PATH: The file path to the LC3 program to execute"
    );
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        print_usage(&args[0]);
        exit(1);
    }

    let file_path = Path::new(&args[1]);
    let mut vm = Lc3Vm::new();
    if let Err(e) = vm.load_program(file_path) {
        eprintln!("Failed to load LC3 program: {e}")
    };
    vm.run();
    println!("=====Program execution complete=====");
}
