pub struct Memory {
    pub data: Vec<i32>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn load(&self, address: usize) -> i32 {
        self.data[address]
    }

    pub fn store(&mut self, address: usize, value: i32) {
        self.data[address] = value;
    }
}
