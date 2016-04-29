use state::State;
use std::char;
use std::io;
use std::io::Read;
use std::ascii::AsciiExt;

pub fn run(code: &str) {
    let mut state = State::new();
    let mut loops: Vec<usize> = vec![];
    let mut skip = false;
    let mut skip_index: usize = 0;
    let mut i: usize = 0;
    let buffer: Vec<char> = code.chars().map(|x| if x.is_ascii() { x } else { '_' }).collect(); // filter out characters larger than 8-bit to simplify indexing
     while i < buffer.len() {
        let instruction = buffer[i];
        //println!("position: {}, state: {}; executing now: {}", i, state, instruction);
        if skip {
            if instruction == '[' {
                loops.push(i);
            } else if instruction == ']' {
                if loops.pop().unwrap() == skip_index {
                    skip = false;
                    i = skip_index;
                    continue;
                }
            }
        } else {
          match instruction {
            '<' => state.decrement_pointer(),
            '>' => state.increment_pointer(),
            '+' => state.increment_value(),
            '-' => state.decrement_value(),
            '.' => print!("{}", char::from_u32(state.get_value() as u32).unwrap()),
            ',' => {
                let stdin = io::stdin();
                let input = stdin.lock().bytes().next().unwrap_or(Ok(0)).unwrap(); // EOF = \0
                state.set_value(input as i32);
            },
            '[' => {
                if state.get_value() == 0 {
                    skip = true;
                    skip_index = i;
                } else {
                    loops.push(i);
                }
            },
            ']' => {
                let start = loops.pop().unwrap();
                if state.get_value() != 0 {
                    i = start;
                    continue;
                }
            },
            _ => {}
          }
        }
        i += 1;
    }
}
