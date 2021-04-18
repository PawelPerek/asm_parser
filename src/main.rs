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
enum Instructions {
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

fn parse(asm: std::io::BufReader<std::fs::File>) -> [Instructions; 16] {
    let mut ptr = 0;

    let mut instructions = [Instructions::CONST(0); 16];

    for line in asm.lines().map(|line| line.unwrap()) {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().or_else(|| Some("")).unwrap();
        let payload = parts.collect::<Vec<&str>>().join("");
        
        let instruction = match instruction.to_lowercase().as_str() {
            "j" => {
                let address = payload.parse().expect("Wrong payload for jmp instruction");
                Instructions::J(Address(address))
            },
            "add" => {
                let mut regs = payload.split(",");

                let reg = match (regs.nth(0).expect("Wrong register 0"), regs.nth(0).expect("Wrong register 1")) {
                    ("r0", "r1") => Register::R0,
                    ("r1", "r0") => Register::R1,
                    _ => panic!("Wrong arguments supplied into ADD instruction")
                };

                Instructions::ADD(reg)
            },
            "mul" => {
                let mut regs = payload.split(",");

                let reg = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    ("r0", "r1") => Register::R0,
                    ("r1", "r0") => Register::R1,
                    _ => panic!("Wrong arguments supplied into MUL instruction")
                };

                Instructions::MUL(reg)
            },
            "bigmul" => {
                assert_eq!(payload, "");
                Instructions::BIGMUL
            },
            "mov" => {
                let mut regs = payload.split(",");

                let (tg, src) = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    ("r0", "r1") => (Register::R0, Source::Register(Register::R1)),
                    ("r1", "r0") => (Register::R1, Source::Register(Register::R0)),
                    ("r0", imm) => (Register::R0, Source::Immediate(imm.parse().expect("Can't parse immediate value"))),
                    ("r1", imm) => (Register::R1, Source::Immediate(imm.parse().expect("Can't parse immediate value"))),
                    _ => panic!("Wrong arguments supplied into MUL instruction")
                };

                Instructions::MOV(tg, src)
            },
            "ld" => {
                let mut regs = payload.split(",");

                let (tg, src) = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    ("r0", imm) => (Register::R0, Address(imm.parse().expect("Can't parse immediate value"))),
                    ("r1", imm) => (Register::R1, Address(imm.parse().expect("Can't parse immediate value"))),
                    _ => panic!("Wrong arguments supplied into LD instruction")
                };

                Instructions::LD(tg, src)
            },
            "st" => {
                let mut regs = payload.split(",");

                let (tg, src) = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    ("r0", imm) => (Register::R0, Address(imm.parse().expect("Can't parse immediate value"))),
                    ("r1", imm) => (Register::R1, Address(imm.parse().expect("Can't parse immediate value"))),
                    _ => panic!("Wrong arguments supplied into ST instruction")
                };

                Instructions::ST(tg, src)
            },
            "db" => {
                let mut regs = payload.split(",");

                let (addr, val) = match (regs.nth(0).unwrap(), regs.nth(0).unwrap()) {
                    (addr, val) => (Address(addr.parse().expect("Can't parse address")), val.parse().expect("Can't parse constant value"))
                };

                Instructions::DB(addr, val)
            },
            "" => continue,
            instr @ _ => panic!("Unrecognised instruction: {}", instr)
        };

        match instruction {
            Instructions::DB(addr, val) => (
                instructions[addr.0] = Instructions::CONST(val)
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

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(&path).expect(format!("Failed to open file: {}", path).as_str());
    let reader = std::io::BufReader::new(file);

    let instructions = parse(reader);

    print!("{:?}", instructions);
}