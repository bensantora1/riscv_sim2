pub struct Cpu {
    pub registers: [i32; 32],
    pub pc: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
        }
    }
}
