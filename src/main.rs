#![feature(io, env)]

use std::env;

mod repl;

fn main() {
    let mut arguments = env::args();
    arguments.next();
    match arguments.next() {
        Some(name) => println!("file: {}", name.into_string().unwrap()),
        None => repl::Repl::run(),
    }
}
