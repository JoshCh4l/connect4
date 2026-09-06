#[derive(PartialEq)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn value(&self) -> u32 {
        match self {
            Player::One => 1,
            Player::Two => 2,
        }
    }

    pub fn switch(&mut self) {
        *self = match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
    }
}
