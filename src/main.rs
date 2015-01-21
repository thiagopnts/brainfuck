#![allow(unstable)]

use std::os;

mod repl;

fn main() {
    let arguments = os::args();
    match arguments.get(1) {
        Some(ref name) => repl::Repl::evaluate(File::new(&Path::new(name))),
        None => repl::Repl::run(),
    }
}
