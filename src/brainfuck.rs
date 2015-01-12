use std::io::stdin;
use std::io::stdio::StdinReader;

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
    content: Box<StdinReader>,
    pointer: i64,
}

impl Brainfuck {
    pub fn new(content: Box<StdinReader>) -> Brainfuck {
        let v = (0 .. 30000).map(|_| 0).collect();
        Brainfuck { memory: v, content: content, pointer: 0 }
    }
    pub fn evaluate(&mut self) {
        let program:Vec<u8> = self.content.read_until('\n' as u8).ok().expect("Couldn't load the entire program to memory");
        let mut program_counter = 0;
        while program_counter < program.len() {
            match program[program_counter].clone().to_op() {
                    Op::Left      => self.left(),
                    Op::Right     => self.right(),
                    Op::Increment => self.increment(),
                    Op::Decrement => self.decrement(),
                    Op::Output    => self.output(),
                    Op::Input     => self.input(),
                    Op::Jump      => {
                        let mut bal: i32 = 1;
                        if self.memory[self.pointer as usize] == 0u8 {
                            loop {
                                program_counter += 1;
                                let c = program[program_counter as usize] as char;
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
                    },
                    Op::JumpBack  => {
                        let mut bal: i32 = 0;
                        loop {
                            let c = program[program_counter as usize] as char;
                            if c == '[' {
                                bal += 1;
                            } else if c == ']' {
                                bal -= 1;
                            }
                            program_counter -= 1;
                            if bal == 0 {
                                break;
                            }
                        }
                    }
                    Op::Unknown => self.unknown(program[program_counter]),
            }
            program_counter += 1;
        }
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

    fn jump(&mut self, reader: Box<Reader>) {
    }

    fn jump_back(&mut self) {

    }

    fn unknown(&self, byte: u8) {
        if byte != 10 {
            println!("\nUnknown OP: {}", byte);
        }
    }
}
