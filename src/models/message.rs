#[derive(Clone, Debug)]
pub struct Message {
    pub name: String,
    pub text: String,
}

impl Message {
    pub fn new(name: String, text: String) -> Self {
        Message { name, text }
    }
}
