use std::num::Wrapping;
use std::fmt;

pub struct State {
    pointer: i32,
    memory: Vec<Wrapping<i32>>
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<pointer address: {}, value: {}, length: {}>", self.get_pointer(), self.get_value(), self.memory.len())
    }
}

impl State {
    pub fn new() -> State { 
        State { pointer: 0, memory: vec![Wrapping(0)] }
    }
    
    fn add_to_pointer(&mut self, value: i32) {
        self.pointer = self.pointer + value;
        while self.pointer as usize >= self.memory.len() {
            self.memory.push(Wrapping(0));
        }
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
