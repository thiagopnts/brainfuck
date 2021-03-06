use std::old_io::stdin;
use std::old_io::stdio::StdinReader;

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
    program: Vec<u8>,
    pointer: i64,
}

impl Brainfuck {
    pub fn new(content: Box<StdinReader>) -> Brainfuck {
        let v = (0 .. 30000).map(|_| 0).collect();
        let mut content = content;
        let program: Vec<u8> = content.read_until('\n' as u8).ok().expect("Couldn't load the entire program to memory");
        Brainfuck { memory: v, program: program, pointer: 0 }
    }
    pub fn evaluate(&mut self) {
        let mut program_counter = 0;
        while program_counter < self.program.len() {
            match self.program[program_counter].to_op() {
                    Op::Left      => self.left(),
                    Op::Right     => self.right(),
                    Op::Increment => self.increment(),
                    Op::Decrement => self.decrement(),
                    Op::Output    => self.output(),
                    Op::Input     => self.input(),
                    Op::Jump      => self.jump(&mut program_counter),
                    Op::JumpBack  => self.jump_back(&mut program_counter),
                    Op::Unknown => self.unknown(self.program[program_counter]),
            }
            program_counter += 1;
        }
        println!("");
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
        print!("{}", (self.memory[self.pointer as usize]) as char);
    }

    fn input(&mut self) {
        self.memory[self.pointer as usize] = stdin().read_char().ok().expect("Error reading user input") as u8;
    }

    fn jump(&mut self, program_counter: &mut usize) {
        let mut bal = 1i32;
        if self.memory[self.pointer as usize] == 0u8 {
            loop {
                *program_counter += 1;
                let c = self.program[*program_counter] as char;
                if c == '[' {
                    bal += 1;
                } else if c == ']' {
                    bal -= 1;
                }
                if bal == 0 {
                    break;
                }
            }
        }
    }

    fn jump_back(&mut self, program_counter: &mut usize) {
        let mut bal = 0i32;
        loop {
            let c = self.program[*program_counter] as char;
            if c == '[' {
                bal += 1;
            } else if c == ']' {
                bal -= 1;
            }
            *program_counter -= 1;
            if bal == 0 {
                break;
            }
        }
    }

    fn unknown(&self, byte: u8) {
        if byte != 10 {
            println!("\nUnknown OP: {}", byte);
        }
    }
}
