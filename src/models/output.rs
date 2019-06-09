#[derive(Debug,Clone)]
pub enum Output {
    True(u32),
    False(u32)
}
impl Output {
    pub fn invert(&self) -> Output {
        match self {
            Output::True(delay) => Output::False(delay.clone()),
            Output::False(delay) => Output::True(delay.clone())
        }
    }
    pub fn add_delay(&self) -> Output {
        match self {
            Output::True(delay) => Output::True(delay + 15),
            Output::False(delay) => Output::False(delay + 15)
        }
    }
}
