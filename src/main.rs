// Brainfuck    C
//     >      ++ptr
//     <      --ptr
//     +      ++*ptr
//     -      --*ptr
//     .      fputc(*ptr, stdout);
//     ,      *ptr = fgetc(stdin);
//     [      while(*ptr) {
//     ]      }

enum OpType {
    OpShl = '<' as isize,
    OpShr = '>' as isize,
    OpInc = '+' as isize,
    OpDec = '-' as isize,
    OpOut = '.' as isize,
    OpIn  = ',' as isize,
    OpJz  = '[' as isize,
    OpJnz = ']' as isize
}

impl OpType {
    fn to_string(&self) -> String {
        match self {
            OpType::OpShl => String::from("OP_SHL"),
            OpType::OpShr => String::from("OP_SHR"),
            OpType::OpInc => String::from("OP_INC"),
            OpType::OpDec => String::from("OP_DEC"),
            OpType::OpOut => String::from("OP_OUT"),
            OpType::OpIn  => String::from("OP_IN"),
            OpType::OpJz  => String::from("OP_JZ"),
            OpType::OpJnz => String::from("OP_JNZ"),
       }
    }

    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '<' => Some(OpType::OpShl),
            '>' => Some(OpType::OpShr),
            '+' => Some(OpType::OpInc),
            '-' => Some(OpType::OpDec),
            '.' => Some(OpType::OpOut),
            ',' => Some(OpType::OpIn),
            '[' => Some(OpType::OpJz),
            ']' => Some(OpType::OpJnz),
             _  => None
        }
    }
}

struct Instruction {
    // meaning of operand for each instruction
    // for OpJz/OpJnz, it means the jump address
    // for others, it means repeat times
    opcode: OpType,
    operand: usize
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}\t{}", self.opcode.to_string(), self.operand)
    }
}

struct BfVm {
    stack: Vec<usize>,       // address stack
    insts: Vec<Instruction>, // instruction set
    dp: usize,               // data pointer
    pc: usize,               // program counter
    memory: Vec<u8>,
}

impl Default for BfVm {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            insts: Vec::new(),
            dp: 0,
            pc: 0,
            memory: vec![0; 30000]
        }
    }
}

fn read_char() -> std::io::Result<char> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    match input.trim().chars().next() {
        Some(ch) => Ok(ch),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "No input"
        )),
    }
}

fn generate_ir(codes: &Vec<char>, vm: &mut BfVm) {
    let mut ptr = 0;

    while ptr < codes.len() {
        let ch = codes[ptr];
        match ch {
            '<' | '>' | '+' | '-' | '.' | ',' => {
                ptr += 1;
                let mut count: usize = 1;
                while ptr < codes.len() && ch == codes[ptr] {
                    count += 1;
                    ptr += 1;
                }
                let inst = Instruction {
                    opcode: OpType::from_char(ch).expect("invalid opcode"),
                    operand: count,
                };
                vm.insts.push(inst);
            },
            '[' => {
                ptr += 1;
                let loop_start_addr = vm.insts.len(); // record the address of current instruction '['
                let inst = Instruction {
                    opcode: OpType::from_char(ch).expect("invalid opcode"),
                    operand: 0, // backpatching when we match the closed paren
                };
                vm.stack.push(loop_start_addr);
                vm.insts.push(inst);
            },
            ']' => {
                ptr += 1;
                let loop_start_addr = vm.stack.pop().expect("stack underflow");
                let inst = Instruction {
                    opcode: OpType::from_char(ch).expect("invalid opcode"),
                    operand: loop_start_addr + 1, // skip instruction OpJz
                };
                vm.insts.push(inst);
                let loop_end_addr = vm.insts.len();
                vm.insts[loop_start_addr].operand = loop_end_addr;
            },
             _  => ptr += 1,
        }
    }

    if !vm.stack.is_empty() {
        panic!("unbalanced paren");
    }
}

fn interpret(vm: &mut BfVm) -> std::io::Result<()> {
    while vm.pc < vm.insts.len() {
        let inst = &vm.insts[vm.pc];
        match inst.opcode {
            OpType::OpShl => {
                if vm.dp < inst.operand {
                    panic!("data pointer undeflow");
                }
                vm.dp -= inst.operand;
                vm.pc += 1;
            },
            OpType::OpShr => {
                if vm.dp + inst.operand >= vm.memory.len() {
                    panic!("data pointer overflow");
                }
                vm.dp += inst.operand;
                vm.pc += 1;
            },
            OpType::OpInc => {
                vm.memory[vm.dp] = vm.memory[vm.dp].wrapping_add(inst.operand as u8);
                vm.pc += 1;
            },
            OpType::OpDec => {
                vm.memory[vm.dp] = vm.memory[vm.dp].wrapping_sub(inst.operand as u8);
                vm.pc += 1;
            },
            OpType::OpOut => {
                for _ in 0..inst.operand {
                    print!("{}", vm.memory[vm.dp] as char);
                }
                vm.pc += 1;
            },
            OpType::OpIn  => {
                for _ in 0..inst.operand {
                    vm.memory[vm.dp] = read_char()? as u8;
                }
                vm.pc += 1;
            },
            OpType::OpJz  => {
                if vm.memory[vm.dp] == 0 {
                    vm.pc = inst.operand;
                } else {
                    vm.pc += 1;
                }
            },
            OpType::OpJnz => {
                if vm.memory[vm.dp] != 0 {
                    vm.pc = inst.operand;
                } else {
                    vm.pc += 1;
                }
            },
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <bf-file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let content = std::fs::read_to_string(filename)?;
    let codes: Vec<char> = content.chars().collect();
    let mut vm = BfVm::default();

    generate_ir(&codes, &mut vm);

    // for inst in &vm.insts {
    //     println!("{}", inst);
    // }

    interpret(&mut vm)?;

    Ok(())
}
