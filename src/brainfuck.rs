use std::io::{stdin, Reader};
use std::ascii::AsciiExt;

#[deriving(PartialEq)]
enum Op {
    Left      = 60,
    Right     = 62,
    Increment = 43,
    Decrement = 45,
    Output    = 46,
    Input     = 44,
    Jump      = 91,
    JumpBack  = 93,
    Unknown,
}

trait ToOp {
    fn to_op(&self) -> Op;
}

impl ToOp for u8 {
    fn to_op(&self) -> Op {
        match *self {
            60 => Op::Left,
            62 => Op::Right,
            43 => Op::Increment,
            45 => Op::Decrement,
            46 => Op::Output,
            44 => Op::Input,
            91 => Op::Jump,
            93 => Op::JumpBack,
            _  => Op::Unknown,
        }
    }
}

pub struct Brainfuck {
    memory: Vec<u8>,
    content: Box<Reader + 'static>,
    pointer: i64,
}

impl Brainfuck {
    pub fn new(content: Box<Reader + 'static>) -> Brainfuck {
        let mut v = vec!();
        for i in range(0, 10000) {
            v.push(64);
        }
        Brainfuck { memory: v, content: content, pointer: 0 }
    }
    pub fn evaluate(&mut self) {
        'eval: loop {
            match self.content.read_byte() {
                Ok(byte) => match byte.to_op() {
                    Op::Left      => self.left(),
                    Op::Right     => self.right(),
                    Op::Increment => self.increment(),
                    Op::Decrement => self.decrement(),
                    Op::Output    => self.output(),
                    Op::Input     => self.input(),
                    Op::Jump      => self.jump(),
                    Op::JumpBack  => self.jump_back(),
                    Op::Unknown => self.unknown(byte),
                },
                Err(err) => break 'eval
            }
        }
        println!("");
        println!("done evaluating");
    }

    fn left(&mut self) {
        self.pointer -= 1;
    }

    fn right(&mut self) {
        self.pointer += 1;
    }

    fn increment(&mut self) {
        self.memory[self.pointer as usize] += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.pointer as usize] -= 1;
    }

    fn output(&self) {
        print!("{}", (self.memory[self.pointer as usize].to_ascii_lowercase()) as char);
    }

    fn input(&mut self) {
        self.memory[self.pointer as usize] = stdin().read_char().ok().expect("Error reading user input") as u8;
    }

    fn jump(&mut self) {
        let mut bal = 1;
        if self.memory[self.pointer as usize] as char == '\0' {
            'foo: loop {
                self.pointer += 1;
                let c = self.memory[self.pointer as usize] as char;
                if c == '[' {
                    bal += 1;
                } else if c == ']' {
                    bal -= 1;
                }
                if bal == 0 {
                    break 'foo
                }
            }
        }
    }

    fn jump_back(&mut self) {
        let mut bal = 0;
        if self.memory[self.pointer as usize] as char == '\0' {
            'foo: loop {
                let c = self.memory[self.pointer as usize] as char;
                if c == '[' {
                    bal += 1;
                } else if c == ']' {
                    bal -= 1;
                }
                self.pointer -= 1;
                if bal == 0 {
                    break 'foo
                }
            }
        }
    }

    fn unknown(&self, byte: u8) {
        println!("\nUnknown OP: {}", byte as char);
    }
}
