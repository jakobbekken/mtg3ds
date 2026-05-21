pub const STARTING_LIFE: i32 = 20;

pub struct Player {
    pub life: i32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            life: STARTING_LIFE,
        }
    }

    pub fn reset(&mut self) {
        self.life = STARTING_LIFE;
    }
}
