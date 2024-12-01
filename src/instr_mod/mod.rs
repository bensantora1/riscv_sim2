#[derive(Debug)] // Enables `Debug` trait for `Instruction`
pub enum Instruction {
    Add(usize, usize, usize),
    Sub(usize, usize, usize),
    Load(usize, usize),
    Store(usize, usize),
    Jump(usize),
    Halt,
}
