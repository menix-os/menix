#[derive(Debug)]
pub struct Thread {
    id: usize,
}

impl Default for Thread {
    fn default() -> Self {
        Self::new()
    }
}

impl Thread {
    pub fn new() -> Self {
        Self { id: 0 }
    }
}
