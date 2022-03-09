use std::env;
use std::io::BufReader;
use std::{fs::File, io::Read};

mod interpreter;

fn read_file(path: &String) -> String {
    let mut res = String::new();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("{}", e),
    };

    let mut fin = BufReader::new(file);
    fin.read_to_string(&mut res).unwrap();

    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && args[1] == "--no-jit" {
        println!("no compile");
        interpreter::interpret(&read_file(&args[2]));
    } else if args.len() > 1 {
        println!("compile now");
        interpreter::interpret(&read_file(&args[1]));
        //let mut j = jit_compiler_x64::JitCompiler::new();
        //j.compile_and_run(&read_file(&args[1]));
    } else {
        println!("Parama Error! Use: {} [--no-jit] program.bf", args[0]);
    }
}
