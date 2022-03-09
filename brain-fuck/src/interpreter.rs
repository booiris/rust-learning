use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

const MEM_MAX: usize = 10;

fn exec(
    code: &Vec<char>,
    memory: &mut Vec<u8>,
    code_p: &mut usize,
    mem_p: &mut usize,
    brackets_cache: &mut HashMap<usize, usize>,
) -> bool {
    match code[*code_p] {
        '>' => {
            *mem_p += 1;
            if *mem_p >= MEM_MAX {
                println!("Error: memory overflow!");
                return false;
            }
        }
        '<' => {
            if *mem_p == 0 {
                println!("Error: memory low overflow!");
                return false;
            }
            *mem_p -= 1;
        }
        '+' => {
            memory[*mem_p] = memory[*mem_p].wrapping_add(1);
        }
        '-' => {
            memory[*mem_p] = memory[*mem_p].wrapping_sub(1);
        }
        '.' => {
            print!("{}", memory[*mem_p] as char);
            io::stdout().flush().unwrap();
        }
        ',' => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            memory[*mem_p] = input.chars().next().unwrap() as u8;
        }
        '[' => {
            if memory[*mem_p] == 0 {
                *code_p = *brackets_cache.get(code_p).unwrap();
            }
        }
        ']' => {
            if memory[*mem_p] != 0 {
                *code_p = *brackets_cache.get(code_p).unwrap();
            }
        }
        _ => (),
    }
    *code_p += 1;
    return true;
}

fn match_brackets(code: &Vec<char>, brackets_cache: &mut HashMap<usize, usize>) -> bool {
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..code.len() {
        match code[i] {
            '[' => {
                stack.push(i);
            }
            ']' => match stack.pop() {
                Some(x) => {
                    brackets_cache.insert(x, i);
                    brackets_cache.insert(i, x);
                }
                None => {
                    return false;
                }
            },
            _ => (),
        }
    }
    if !stack.is_empty() {
        return false;
    }
    return true;
}

pub fn interpret(code_buffer: &String) {
    let mut memory: Vec<u8> = vec![0; MEM_MAX];
    let mut mem_pointer: usize = 0;

    let code: Vec<char> = code_buffer.chars().collect();
    let mut code_pointer: usize = 0;
    let mut brackets_cache: HashMap<usize, usize> = HashMap::new();
    if !match_brackets(&code, &mut brackets_cache) {
        println!("Error: brackets not match!");
        return;
    }

    loop {
        if !exec(
            &code,
            &mut memory,
            &mut code_pointer,
            &mut mem_pointer,
            &mut brackets_cache,
        ) {
            return;
        };
        if code_pointer >= code.len() {
            break;
        }
    }
}
