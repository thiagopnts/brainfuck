use std::io;

pub struct Repl;

impl Repl {
    pub fn run() {
        let mut reader = io::stdin();
        println!("\n\n\nBrainfuck REPL version 0.0.1");
        loop {
            print!("> ");
            let code = reader.read_line().ok().expect("Failed to read line");
            println!("{}", code);
        }
    }
}
