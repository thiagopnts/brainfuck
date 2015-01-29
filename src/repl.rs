#![allow(unstable)]

use std::old_io;

#[path = "brainfuck.rs"]
mod interpreter;

pub struct Repl;

impl Repl {
    pub fn run() {
        println!("\n\n\nBrainfuck REPL version 0.0.1");
        loop {
            print!("> ");
            let reader = old_io::stdin();
            let mut brainfuck = interpreter::Brainfuck::new(Box::new(reader));
            brainfuck.evaluate();
        }
    }
}
