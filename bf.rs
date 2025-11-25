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

static mut DP: usize = 0; // data pointer
static mut MEMORY: [u8; 1000] = [0; 1000];

fn run(codes: &[char]) -> std::io::Result<()> {
    let mut pc = 0; // program counter

    unsafe {
        while pc < codes.len() {
            match codes[pc] {
                '>' => DP += 1,
                '<' => DP -= 1,
                '+' => MEMORY[DP] = MEMORY[DP].wrapping_add(1),
                '-' => MEMORY[DP] = MEMORY[DP].wrapping_sub(1),
                '.' => print!("{}", MEMORY[DP] as char),
                ',' => MEMORY[DP] = read_char()? as u8,
                '[' => {
                    let mut loop_layer = 1;
                    let mut loop_end = pc + 1; // skip '['
                    while loop_layer > 0 {
                        match codes[loop_end] {
                            '[' => loop_layer += 1,
                            ']' => loop_layer -= 1,
                             _  => {}
                        }
                        loop_end += 1; // point to next pos of ']'
                    }
                    while MEMORY[DP] != 0 {
                        let loop_codes = &codes[pc+1..loop_end-1];
                        run(loop_codes)?;
                    }
                    pc = loop_end;
                    continue;
                },
                ']' => {},
                 _  => {},
            };

            pc += 1;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test/helloworld.bf")?;
    let codes: Vec<char> = content.chars().collect();
    run(&codes)?;
    Ok(())
}
