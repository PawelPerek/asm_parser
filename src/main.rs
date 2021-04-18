use std::io::BufRead;

#[derive(Copy, Clone)]
struct Address(u8);

#[derive(Copy, Clone)]
enum Target {
    Register(u8),
    Immediate(u8)
} 

#[derive(Copy, Clone)]
enum Instructions {
    J(Address),
    ADD(Target),
    MUL(Target),
    BIGMUL,
    LD(Address),
    ST(Address)
}

fn parse(asm: std::io::BufReader<std::fs::File>) -> [Instructions; 16] {
    let mut ptr = 0;

    let intructions = [Instructions::J(Address(0)); 16];

    for line in asm.lines().map(|line| line.unwrap()) {
        let mut parts = line.split_whitespace();
        let instruction = parts.next().unwrap();
        let payload = parts.collect::<Vec<&str>>().join("");
    }

    intructions
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(path).expect(format!("Failed to open file, {}", path));
    let reader = std::io::BufReader::new(file);


    let instructions = parse(reader);
}