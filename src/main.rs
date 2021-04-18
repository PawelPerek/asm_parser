use std::io::BufRead;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Address(usize);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Register{
    R0, 
    R1
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Source {
    Register(Register),
    Immediate(u8)
} 

#[derive(Copy, Clone, Debug, PartialEq)]
enum Instruction {
    J(Address),
    ADD(Register),
    MUL(Register),
    BIGMUL,
    MOV(Register, Source),
    LD(Register, Address),
    ST(Register, Address),
    DB(Address, u8),
    CONST(u8)
}

fn parse(asm: std::io::BufReader<std::fs::File>) -> [Instruction; 16] {
    let mut ptr = 0;

    let mut instructions = [Instruction::CONST(0); 16];

    for line in asm.lines().map(|line| line.unwrap()) {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().or_else(|| Some("")).unwrap();
        let payload = parts.collect::<Vec<&str>>().join("");
        
        let instruction = match instruction.to_lowercase().as_str() {
            "j" => {
                let address = payload.parse().expect("Wrong payload for jmp instruction");
                Instruction::J(Address(address))
            },
            "add" => {
                let mut regs = payload.split(",");

                let reg = match (regs.nth(0).expect("Wrong register 0"), regs.nth(0).expect("Wrong register 1")) {
                    ("r0", "r1") => Register::R0,
                    ("r1", "r0") => Register::R1,
                    _ => panic!("Wrong arguments supplied into ADD instruction")
                };

                Instruction::ADD(reg)
            },
            "mul" => {
                let mut regs = payload.split(",");

                let reg = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    ("r0", "r1") => Register::R0,
                    ("r1", "r0") => Register::R1,
                    _ => panic!("Wrong arguments supplied into MUL instruction")
                };

                Instruction::MUL(reg)
            },
            "bigmul" => {
                assert_eq!(payload, "");
                Instruction::BIGMUL
            },
            "mov" => {
                let mut regs = payload.split(",");

                let (tg, src) = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    ("r0", "r1") => (Register::R0, Source::Register(Register::R1)),
                    ("r1", "r0") => (Register::R1, Source::Register(Register::R0)),
                    ("r0", imm) => (Register::R0, Source::Immediate(imm.parse().expect("Can't parse immediate value"))),
                    ("r1", imm) => (Register::R1, Source::Immediate(imm.parse().expect("Can't parse immediate value"))),
                    _ => panic!("Wrong arguments supplied into MOV instruction")
                };

                Instruction::MOV(tg, src)
            },
            "ld" => {
                let mut regs = payload.split(",");

                let (tg, src) = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    ("r0", imm) => (Register::R0, Address(imm.parse().expect("Can't parse immediate value"))),
                    ("r1", imm) => (Register::R1, Address(imm.parse().expect("Can't parse immediate value"))),
                    _ => panic!("Wrong arguments supplied into LD instruction")
                };

                Instruction::LD(tg, src)
            },
            "st" => {
                let mut regs = payload.split(",");

                let (tg, src) = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    ("r0", imm) => (Register::R0, Address(imm.parse().expect("Can't parse immediate value"))),
                    ("r1", imm) => (Register::R1, Address(imm.parse().expect("Can't parse immediate value"))),
                    _ => panic!("Wrong arguments supplied into ST instruction")
                };

                Instruction::ST(tg, src)
            },
            "db" => {
                let mut regs = payload.split(",");

                let (addr, val) = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    (addr, val) => (Address(addr.parse().expect("Can't parse address")), val.parse().expect("Can't parse constant value"))
                };

                Instruction::DB(addr, val)
            },
            "" => continue,
            instr @ _ => panic!("Unrecognised instruction: {}", instr)
        };

        match instruction {
            Instruction::DB(addr, val) => (
                instructions[addr.0] = Instruction::CONST(val)
            ),
            instr @ _ => {
                instructions[ptr] = instr;
                ptr += 1;
            }
        }
        
        if ptr == 16 {
            panic!("Memory overflow")
        }
    }

    instructions
}

fn output(instructions: [Instruction; 16]) -> String {
    instructions.iter().map(|el| match el {
        Instruction::J(Address(addr)) => 0b0000_0000 | *addr as u8,
        Instruction::ADD(reg) => if reg == &Register::R0 {0b0010_0000} else {0b0011_0000},
        Instruction::MUL(reg) => if reg == &Register::R0 {0b0100_0000} else {0b0101_0000},
        Instruction::BIGMUL => 0b0001_0000,
        Instruction::MOV(reg, src) => 0b1000_0000 | ((*reg as u8) << 4) | match src {
            Source::Register(_) => 0b0010_0000,
            Source::Immediate(val) => *val
        },
        Instruction::LD(reg, Address(addr)) => 0b1100_0000 | ((*reg as u8) << 4) | (*addr as u8),
        Instruction::ST(reg, Address(addr)) => 0b1110_0000 | ((*reg as u8) << 4) | (*addr as u8),
        Instruction::CONST(val) => *val,
        _ => panic!("Parser error")
    }).map(|el| format!("{:02X}", el)).collect::<Vec<String>>().join(",")
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(&path).expect(format!("Failed to open file: {}", path).as_str());
    let reader = std::io::BufReader::new(file);

    let instructions = parse(reader);

    print!("{}", output(instructions));
}