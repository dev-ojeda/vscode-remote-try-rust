#[derive(Debug)]
pub struct Task {
    pub tittle: String,
    pub done: bool,
    pub desc: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            tittle: Default::default(),
            done: Default::default(),
            desc: Default::default(),
        }
    }
}

impl Task {
    pub fn new(tittle: impl Into<String>) -> Self {
        Self {
            tittle: tittle.into(),
            done: false,
            desc: None,
        }
    }
}
