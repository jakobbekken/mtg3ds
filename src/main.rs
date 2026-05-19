mod game;
mod player;
mod ui;

use ctru::prelude::*;
use game::Game;
use ui::draw_player;

fn main() {
    let gfx = Gfx::new().expect("Failed to init graphics");
    let mut hid = Hid::new().expect("Failed to init HID");
    let apt = Apt::new().expect("Failed to init APT");

    let top = Console::new(gfx.top_screen.borrow_mut());
    let bot = Console::new(gfx.bottom_screen.borrow_mut());

    let mut game = Game::new();
    let mut dirty = true;

    while apt.main_loop() {
        hid.scan_input();
        let keys = hid.keys_down();

        if keys.intersects(KeyPad::START) {
            break;
        }

        if keys.intersects(KeyPad::SELECT) {
            game.reset_all();
            dirty = true;
        }
        if keys.intersects(KeyPad::L) {
            game.select(0);
            dirty = true;
        }
        if keys.intersects(KeyPad::R) {
            game.select(1);
            dirty = true;
        }
        if keys.intersects(KeyPad::UP) {
            game.adjust_life(1);
            dirty = true;
        }
        if keys.intersects(KeyPad::DOWN) {
            game.adjust_life(-1);
            dirty = true;
        }

        if dirty {
            draw_player(&top, &game.players[0], game.selected == 0);
            draw_player(&bot, &game.players[1], game.selected == 1);
            dirty = false;
        }

        gfx.wait_for_vblank();
    }
}
