#[repr(C, packed)]
pub struct Module {
    pub init: fn() -> i32,
    pub exit: Option<fn() -> i32>,
    pub name: [u8; 64],
    pub description: [u8; 64],
    pub num_dependencies: usize,
}
