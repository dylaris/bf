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

#[allow(dead_code)]
struct BfVm {
    stack: Vec<usize>,
    insts: Vec<Instruction>,
    dp: usize,
    pc: usize,
    memory: [u8; 1000]
}

impl Default for BfVm {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            insts: Vec::new(),
            dp: 0,
            pc: 0,
            memory: [0; 1000],
        }
    }
}

#[allow(dead_code)]
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

// static mut DP: usize = 0; // data pointer
// static mut MEMORY: [u8; 1000] = [0; 1000];
//
// fn run(codes: &[char]) -> std::io::Result<()> {
//     let mut pc = 0; // program counter
//
//     unsafe {
//         while pc < codes.len() {
//             match codes[pc] {
//                 '>' => DP += 1,
//                 '<' => DP -= 1,
//                 '+' => MEMORY[DP] = MEMORY[DP].wrapping_add(1),
//                 '-' => MEMORY[DP] = MEMORY[DP].wrapping_sub(1),
//                 '.' => print!("{}", MEMORY[DP] as char),
//                 ',' => MEMORY[DP] = read_char()? as u8,
//                 '[' => {
//                     let mut loop_layer = 1;
//                     let mut loop_end = pc + 1; // skip '['
//                     while loop_layer > 0 {
//                         match codes[loop_end] {
//                             '[' => loop_layer += 1,
//                             ']' => loop_layer -= 1,
//                              _  => {}
//                         }
//                         loop_end += 1; // point to next pos of ']'
//                     }
//                     while MEMORY[DP] != 0 {
//                         let loop_codes = &codes[pc+1..loop_end-1];
//                         run(loop_codes)?;
//                     }
//                     pc = loop_end;
//                     continue;
//                 },
//                 ']' => {},
//                  _  => {},
//             };
//
//             pc += 1;
//         }
//     }
//
//     Ok(())
// }

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

    for inst in &vm.insts {
        println!("{}", inst);
    }

    Ok(())
}
