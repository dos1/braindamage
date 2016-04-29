mod input;
mod interpreter;
use std::io::Read;
use std::env;

fn main() {
    let mut stream = input::Input::from_arg(env::args().nth(1)).unwrap();
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    interpreter::run(&*buffer);
}
