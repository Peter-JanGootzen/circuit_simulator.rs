#[derive(Debug, Clone)]
pub enum Output {
    True,
    False
}
impl Output {
    pub fn invert(&self) -> Output {
        match self {
            Output::True => Output::False,
            Output::False => Output::True
        }
    }
}
