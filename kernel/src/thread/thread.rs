#[derive(Debug)]
pub struct Thread {
    id: usize,
}

impl Thread {
    pub fn new() -> Self {
        Self { id: 0 }
    }
}
