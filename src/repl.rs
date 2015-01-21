#![allow(unstable)]

use std::io;

#[path = "brainfuck.rs"]
mod interpreter;

pub struct Repl;

impl Repl {
    pub fn evaluate(reader: Reader) {
        let mut brainfuck = interpreter::Brainfuck::new(Box::new(reader));
        brainfuck.evaluate();
    }

    pub fn run() {
        println!("\n\n\nBrainfuck REPL version 0.0.1");
        loop {
            print!("> ");
            let reader = io::stdin();
            let mut brainfuck = interpreter::Brainfuck::new(Box::new(reader));
            brainfuck.evaluate();
        }
    }
}
