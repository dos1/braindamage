mod input;
mod interpreter;
mod state;
use input::Input;
use std::io::Read;
use std::env;

fn main() {
    let mut stream = Input::from_arg(env::args().nth(1)).unwrap();
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    interpreter::run(&*buffer);
}
