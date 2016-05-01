mod stream;
mod interpreter;
mod state;
use std::fs;
use std::io;
use std::io::Read;
use std::env;

fn main() {
    let mut code = String::new();
    if env::args().len() > 1 {
        fs::File::open(env::args().nth(1).unwrap()).unwrap().read_to_string(&mut code).unwrap();
    } else {
        io::stdin().read_to_string(&mut code).unwrap();
    }
    interpreter::run(&*code, &mut io::stdin(), &mut io::stdout());
}
