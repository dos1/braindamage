use std::char;
use std::io;
use std::io::Read;
use std::num::Wrapping;
use std::fmt;
use std::ascii::AsciiExt;

const MEMORY_SIZE: usize = 100000;

struct State {
    pointer: i32,
    memory: [Wrapping<i32>; MEMORY_SIZE]
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<pointer address: {}, value: {}>", self.get_pointer(), self.get_value())
    }
}

impl State {
    pub fn new() -> State { 
        State { pointer: 0, memory: [Wrapping(0); MEMORY_SIZE] }
    }
    
    fn add_to_pointer(&mut self, value: i32) {
        self.pointer = self.pointer + value;
    }
    
    pub fn increment_pointer(&mut self) {
        self.add_to_pointer(1)
    }
    
    pub fn decrement_pointer(&mut self) {
        self.add_to_pointer(-1)
    }
    
    pub fn set_value(&mut self, value: i32) {
        self.memory[self.pointer as usize] = Wrapping(value);
    }
    
    pub fn get_value(&self) -> i32 {
        self.memory[self.pointer as usize].0
    }
    
    fn get_pointer(&self) -> i32 {
        self.pointer
    }
    
    fn add_to_value(&mut self, value: i32) {
        self.memory[self.pointer as usize] += Wrapping(value);
    }
    
    pub fn increment_value(&mut self) {
        self.add_to_value(1)
    }
    
    pub fn decrement_value(&mut self) {
        self.add_to_value(-1)
    }
}

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
