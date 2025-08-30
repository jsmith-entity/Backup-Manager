#[derive(Clone)]
pub struct TextField {
    pub input: String,
}

impl TextField {
    pub fn new() -> Self {
        return Self {
            input: String::new(),
        };
    }
}
