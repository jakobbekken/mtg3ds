use crate::player::Player;

pub struct Game {
    pub players: [Player; 4],
    pub selected: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: [Player::new(), Player::new(), Player::new(), Player::new()],
            selected: 0,
        }
    }

    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }

    pub fn adjust_life(&mut self, delta: i32) {
        self.players[self.selected].life += delta;
    }

    pub fn commander_damage(&mut self, from: usize) {
        self.players[self.selected].life -= 1;
        self.players[self.selected].commander_damage[from] += 1;
    }

    pub fn ping_all(&mut self) {
        for i in 0..4 {
            if i != self.selected {
                self.players[i].life -= 1;
            }
        }
    }

    pub fn reset_all(&mut self) {
        self.players.iter_mut().for_each(|p| p.reset());
    }
}
