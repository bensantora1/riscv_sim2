pub struct Cache {
    pub data: Vec<Option<i32>>,
    pub valid: Vec<bool>,
    pub lru: Vec<usize>,
}

impl Cache {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![None; size],
            valid: vec![false; size],
            lru: vec![0; size],
        }
    }

    pub fn load(&mut self, address: usize) -> Option<i32> {
        let index = address % self.data.len();
        if self.valid[index] {
            self.lru[index] = self.get_next_lru();
            return self.data[index];
        }
        None
    }

    pub fn store(&mut self, address: usize, value: i32) {
        let index = address % self.data.len();
        self.data[index] = Some(value);
        self.valid[index] = true;
        self.lru[index] = self.get_next_lru();
    }

    fn get_next_lru(&self) -> usize {
        self.lru.iter().max().unwrap_or(&0) + 1
    }
}
