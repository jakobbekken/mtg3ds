pub const STARTING_LIFE: i32 = 40;

pub struct Player {
    pub life: i32,
    pub commander_damage: [i32; 4],
}

impl Player {
    pub fn new() -> Self {
        Self {
            life: STARTING_LIFE,
            commander_damage: [0; 4],
        }
    }

    pub fn reset(&mut self) {
        self.life = STARTING_LIFE;
        self.commander_damage = [0; 4];
    }
}
