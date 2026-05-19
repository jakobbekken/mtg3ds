pub const STARTING_LIFE: i32 = 20;

pub struct Player {
    pub name: &'static str,
    pub life: i32,
}

impl Player {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            life: STARTING_LIFE,
        }
    }

    pub fn reset(&mut self) {
        self.life = STARTING_LIFE;
    }
}
