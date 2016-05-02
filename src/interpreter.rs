use state::State;
use stream::{Input, Output};
use std::io::Read;
use std::ascii::AsciiExt;

struct Routine<'input> {
    code: Vec<char>,
    state: State,
    position: usize,
    loops: Vec<usize>,
    skip: bool,
    skip_index: usize,
    input: &'input mut Input
}

impl<'input> Routine<'input> {
    fn new(code: &str, input: &'input mut Input) -> Routine<'input> {
        let buffer: Vec<char> = code.chars().map(|x| if x.is_ascii() { x } else { '_' }).collect(); // filter out characters larger than 8-bit to simplify indexing
        Routine { code: buffer, position: 0, state: State::new(), loops: vec![], skip: false, skip_index: 0, input: input }
    }
    
    fn exec(&mut self) -> Option<i32> {
        let instruction = self.code[self.position];
        let mut result : Option<i32> = None;
        //println!("position: {}, state: {}; executing now: '{}', skip: {}", self.position, self.state, instruction, self.skip);
        if self.skip {
            if instruction == '[' {
                self.loops.push(self.position);
            } else if instruction == ']' {
                if self.loops.pop().unwrap() == self.skip_index {
                    self.skip = false;
                }
            }
        } else {
          match instruction {
            '<' => self.state.decrement_pointer(),
            '>' => self.state.increment_pointer(),
            '+' => self.state.increment_value(),
            '-' => self.state.decrement_value(),
            '.' => result = Some(self.state.get_value()),
            ',' => {
                let data = self.input.read_value().unwrap_or(0); // EOF = '\0'
                self.state.set_value(data);
            },
            '[' => {
                if self.state.get_value() == 0 {
                    self.skip = true;
                    self.skip_index = self.position;
                }
                self.loops.push(self.position);
            },
            ']' => {
                let start = self.loops.pop().unwrap();
                if self.state.get_value() != 0 {
                    self.position = start;
                    return result;
                }
            },
            _ => {}
          }
        }
        self.position += 1;
        
        return result;
    }
}

impl<'input> Iterator for Routine<'input> {
    type Item = Option<i32>;

    fn next(&mut self) -> Option<Option<i32>> {
        if self.position < self.code.len() {
            Some(self.exec())
        } else {
            None
        }
    }

}

pub fn run(code: &str, input: &mut Input, output: &mut Output) {

    for data in Routine::new(code, input) {
        if data.is_some() {
            output.write_value(data.unwrap()).unwrap();
        }
    }
    
}
