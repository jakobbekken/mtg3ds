mod game;
mod player;
mod ui;

use citro2d::{Citro2d, TextBuf};
use ctru::prelude::*;
use game::Game;

const IDLE_TIMEOUT: u32 = 180;

fn main() {
    let gfx = Gfx::new().expect("Failed to init gfx");
    let mut hid = Hid::new().expect("Failed to init HID");
    let apt = Apt::new().expect("Failed to init APT");

    let c2d = Citro2d::new(&gfx).expect("Failed to init citro2d");
    let text_buf = TextBuf::new(512).expect("Failed to create text buffer");

    let mut game = Game::new();
    let mut dirty = true;
    let mut idle_timer: u32 = 0;
    let mut show_active = false;

    while apt.main_loop() {
        hid.scan_input();
        let keys_down = hid.keys_down();

        if keys_down.intersects(KeyPad::START) {
            break;
        }
        if keys_down.intersects(KeyPad::SELECT) {
            game.reset_all();
            dirty = true;
        }

        if keys_down.intersects(KeyPad::TOUCH) {
            let (x, y) = hid.touch_position();
            let x = x as f32;
            let y = y as f32;

            idle_timer = 0;
            if !show_active {
                show_active = true;
                dirty = true;
            }

            // commander damage buttons
            if y >= 30.0 && y < 75.0 {
                let cmd_btn_w = (ui::BOT_W - 16.0) / 3.0 - 4.0;
                let mut slot = 0;
                for i in 0..4 {
                    if i == game.selected {
                        continue;
                    }
                    let bx = 8.0 + slot as f32 * (cmd_btn_w + 4.0);
                    if x >= bx && x < bx + cmd_btn_w {
                        game.commander_damage(i);
                        dirty = true;
                        break;
                    }
                    slot += 1;
                }
            } else if y >= 83.0 && y < 118.0 {
                if x >= 8.0 && x < ui::BOT_W - 8.0 {
                    game.ping_all();
                    dirty = true;
                }
            } else if y >= 126.0 && y < 166.0 {
                let btn_w = (ui::BOT_W - 20.0) / 4.0 - 4.0;
                if x >= 8.0 && x < 8.0 + btn_w {
                    game.adjust_life(-10);
                    dirty = true;
                } else if x >= 8.0 + btn_w + 4.0 && x < 8.0 + (btn_w + 4.0) * 2.0 {
                    game.adjust_life(-1);
                    dirty = true;
                } else if x >= 8.0 + (btn_w + 4.0) * 2.0 && x < 8.0 + (btn_w + 4.0) * 3.0 {
                    game.adjust_life(1);
                    dirty = true;
                } else if x >= 8.0 + (btn_w + 4.0) * 3.0 && x < ui::BOT_W - 8.0 {
                    game.adjust_life(10);
                    dirty = true;
                }
            } else if y >= 200.0 {
                let player = (x / ui::TAB_W) as usize;
                if player < 4 {
                    game.select(player);
                    dirty = true;
                }
            }
        } else {
            idle_timer += 1;
            if idle_timer >= IDLE_TIMEOUT && show_active {
                show_active = false;
                dirty = true;
            }
        }

        if dirty {
            let lives = [
                game.players[0].life,
                game.players[1].life,
                game.players[2].life,
                game.players[3].life,
            ];
            let cmd_damage = [
                game.players[0].commander_damage,
                game.players[1].commander_damage,
                game.players[2].commander_damage,
                game.players[3].commander_damage,
            ];

            text_buf.clear();
            c2d.frame(|frame| {
                ui::draw_top(
                    frame,
                    c2d.top_screen(),
                    lives,
                    cmd_damage,
                    game.selected,
                    show_active,
                    &text_buf,
                );
                ui::draw_bottom(
                    frame,
                    c2d.bottom_screen(),
                    game.selected,
                    game.players[game.selected].commander_damage,
                    &text_buf,
                );
            });
            dirty = false;
        } else {
            gfx.wait_for_vblank();
        }
    }
}
