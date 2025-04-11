// TODO
pub struct CmdLine<'a> {
    data: &'a str,
}

impl<'a> CmdLine<'a> {
    pub fn new(data: &'a str) -> Self {
        return Self { data };
    }

    pub fn get_bool(name: &str, default: bool) -> Option<bool> {
        todo!()
    }

    pub fn get_usize(name: &str, default: usize) -> Option<usize> {
        todo!()
    }
}
