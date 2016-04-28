use std::char;
use std::io;
use std::io::Read;
use std::num::Wrapping;
use std::fmt;

const MEMORY_SIZE: usize = 100000;

struct State {
    pointer: i32,
    memory: [Wrapping<i8>; MEMORY_SIZE]
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
    
    pub fn set_value(&mut self, value: i8) {
        self.memory[self.pointer as usize] = Wrapping(value);
    }
    
    pub fn get_value(&self) -> i8 {
        self.memory[self.pointer as usize].0
    }
    
    fn get_pointer(&self) -> i32 {
        self.pointer
    }
    
    fn add_to_value(&mut self, value: i8) {
        self.memory[self.pointer as usize] += Wrapping(value);
    }
    
    pub fn increment_value(&mut self) {
        self.add_to_value(1)
    }
    
    pub fn decrement_value(&mut self) {
        self.add_to_value(-1)
    }
}

pub fn run(buffer: String) {
    let mut state = State::new();
    let mut loops: Vec<i32> = vec![];
    let mut skip = false;
    let mut skip_index = -1;
    let mut i = 0;
    while i < buffer.len() {
        let instruction = buffer.chars().nth(i).unwrap();
        //println!("position: {}, state: {}; executing now: {}", i, state, instruction);
        if skip {
            if instruction == '[' {
                loops.push(i as i32);
            } else if instruction == ']' {
                if loops.pop().unwrap() == skip_index {
                    skip = false;
                    i = skip_index as usize;
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
                let input: Option<i8> = io::stdin().bytes().next().and_then(|result| result.ok()).map(|byte| byte as i8);
                state.set_value(input.unwrap());
            },
            '[' => {
                if state.get_value() == 0 {
                    skip = true;
                    skip_index = i as i32;
                } else {
                    loops.push(i as i32);
                }
            },
            ']' => {
                let start = loops.pop().unwrap();
                if state.get_value() != 0 {
                    i = start as usize;
                    continue;
                }
            },
            _ => {}
          }
        }
        i += 1;
    }
}
