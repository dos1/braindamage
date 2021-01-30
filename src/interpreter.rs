use state::State;
use stream::{Input, Output};

struct Routine<'input> {
    code: Vec<char>,
    state: State,
    position: usize,
    loops: Vec<usize>,
    skip: bool,
    skip_index: usize,
    input: &'input mut dyn Input
}

impl<'input> Routine<'input> {
    fn new(code: &str, input: &'input mut dyn Input) -> Routine<'input> {
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

pub fn run(code: &str, input: &mut dyn Input, output: &mut dyn Output) {

    for data in Routine::new(code, input) {
        if data.is_some() {
            output.write_value(data.unwrap()).unwrap();
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::char;
    use stream::*;

    #[test]
    fn print_10() {
        let mut stream = MemoryStream::new();
        run("--++++++><++++++-+.", &mut io::empty(), &mut stream);
        assert_eq!(stream.read_value().unwrap(), 10);
        assert!(stream.read_value().is_err());
    }
    
    #[test]
    fn comments() {
        let mut stream = MemoryStream::new();
        run("C++? I don't like C++; I think Rust is much better than C++! Qt is written in C++ though.", &mut io::empty(), &mut stream);
        assert_eq!(stream.read_value().unwrap(), 8);
        assert!(stream.read_value().is_err());
    }
    
    #[test]
    fn skip_loop() {
        let mut stream = MemoryStream::new();
        run("[.]", &mut io::empty(), &mut stream);
        assert!(stream.read_value().is_err());
    }
    
    #[test]
    fn nested_loops() {
        let mut stream = MemoryStream::new();
        run("+++++>++>++++<<[.>[->[->[]<.]<.]<-]", &mut io::empty(), &mut stream);
        assert_eq!(stream.read_value().unwrap(), 5);
        assert_eq!(stream.read_value().unwrap(), 3);
        assert_eq!(stream.read_value().unwrap(), 2);
        assert_eq!(stream.read_value().unwrap(), 1);
        assert_eq!(stream.read_value().unwrap(), 0);
        assert_eq!(stream.read_value().unwrap(), 1);
        assert_eq!(stream.read_value().unwrap(), 0);
        assert_eq!(stream.read_value().unwrap(), 4);
        assert_eq!(stream.read_value().unwrap(), 3);
        assert_eq!(stream.read_value().unwrap(), 2);
        assert_eq!(stream.read_value().unwrap(), 1);
        assert!(stream.read_value().is_err());
    }
    
    #[test]
    fn loops() {
        let mut stream = MemoryStream::new();
        // from brainfuck stress test: https://github.com/rdebath/Brainfuck/blob/master/bitwidth.b
        run("[-]>[-]++++++++[<+++++++++>-]<.>+++++[<+++++>-]<++++.
	+++++++.><.+++.
	[-] [[-]++>[-]+++++[<++++++>-]<.++>+++++++[<+++++++>-]<.+>+
	+++[<+++++>-]<.+.+++++++++++.------------.---.----.+++.>++++
	++++[<-------->-]<---.>++++[<----->-]<---.[-][]]", &mut io::empty(), &mut stream);
        assert_eq!(char::from_u32(stream.read_value().unwrap() as u32).unwrap(), 'H');
        assert_eq!(char::from_u32(stream.read_value().unwrap() as u32).unwrap(), 'e');
        assert_eq!(char::from_u32(stream.read_value().unwrap() as u32).unwrap(), 'l');
        assert_eq!(char::from_u32(stream.read_value().unwrap() as u32).unwrap(), 'l');
        assert_eq!(char::from_u32(stream.read_value().unwrap() as u32).unwrap(), 'o');
        assert!(stream.read_value().is_err());
        run(">
	    +[>[
		Print the exclamation point
		[-]+++>[-]+++++[<+++2+++>-]<.

	    <[-]>[-]]+<]
	<

	[<[[<[[<[[<[,]]]<]<]<]<][ Deep nesting non-comment comment loop ]]", &mut io::empty(), &mut stream);
        assert_eq!(char::from_u32(stream.read_value().unwrap() as u32).unwrap(), '!');
        assert!(stream.read_value().is_err());
    }
    
    #[test]
    fn eof() {
        let mut stream = MemoryStream::new();
        run("+++.,.", &mut io::empty(), &mut stream);
        assert_eq!(stream.read_value().unwrap(), 3);
        assert_eq!(stream.read_value().unwrap(), 0); // EOF
        assert!(stream.read_value().is_err());
    }
    
    #[test]
    fn io() {
        let mut output_stream = MemoryStream::new();
        let mut input_stream = MemoryStream::new();
        input_stream.write_value(1).unwrap();
        input_stream.write_value(2).unwrap();
        input_stream.write_value(3).unwrap();
        run("---.,>,>,-.<-.<-.", &mut input_stream, &mut output_stream);
        assert_eq!(output_stream.read_value().unwrap(), -3);
        assert_eq!(output_stream.read_value().unwrap(), 2);
        assert_eq!(output_stream.read_value().unwrap(), 1);
        assert_eq!(output_stream.read_value().unwrap(), 0);
        assert!(output_stream.read_value().is_err());
    }
    
    #[test]
    #[should_panic(expected = "tried to set the pointer to negative address")]
    fn negative_index() {
        run("<", &mut io::empty(), &mut io::sink());
    }

}
