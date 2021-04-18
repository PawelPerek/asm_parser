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

fn parse(_asm: std::io::BufReader<std::fs::File>) -> [Instructions; 16] {
    [Instructions::J(Address(0)); 16]
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);


    let instructions = parse(reader);
}