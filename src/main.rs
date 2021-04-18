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

fn main() {
    println!("Hello, world!");
}
