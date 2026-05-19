use crate::player::Player;

pub struct Game {
    pub players: [Player; 2],
    pub selected: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: [
                Player::new("Player 1"),
                Player::new("Player 2"),
            ],
            selected: 0,
        }
    }

    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }

    pub fn adjust_life(&mut self, delta: i32) {
        self.players[self.selected].life += delta;
    }

    pub fn reset_all(&mut self) {
        self.players.iter_mut().for_each(|p| p.reset());
    }
}
