#![feature(allocator_api)]
mod audio;
mod game;
mod player;
mod ui;

use audio::Audio;
use citro2d::{Citro2d, TextBuf};
use ctru::prelude::*;
use game::Game;

const IDLE_TIMEOUT: u32 = 90;

fn main() {
    let gfx = Gfx::new().expect("Failed to init gfx");
    let mut hid = Hid::new().expect("Failed to init HID");
    let apt = Apt::new().expect("Failed to init APT");

    let c2d = Citro2d::new(&gfx).expect("Failed to init citro2d");
    let text_buf = TextBuf::new(512).expect("Failed to create text buffer");

    let audio = Audio::new();
    let mut channel = audio.setup_channel();
    let mut wave = Audio::generate_tone();

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
            let player = if (x as f32) < 160.0 { 0 } else { 1 };
            let delta = if (y as f32) < 120.0 { 1 } else { -1 };
            game.select(player);
            game.adjust_life(delta);
            idle_timer = 0;
            if !show_active {
                show_active = true;
            }
            dirty = true;
            channel.clear_queue();
            let _ = channel.queue_wave(&mut wave);
        } else {
            idle_timer += 1;
            if idle_timer >= IDLE_TIMEOUT && show_active {
                show_active = false;
                dirty = true;
            }
        }

        if dirty {
            text_buf.clear();
            c2d.frame(|frame| {
                ui::draw_top(
                    frame,
                    c2d.top_screen(),
                    game.players[0].life,
                    game.players[1].life,
                    game.selected,
                    show_active,
                    &text_buf,
                );
                ui::draw_bottom(frame, c2d.bottom_screen(), &text_buf);
            });
            dirty = false;
        } else {
            gfx.wait_for_vblank();
        }
    }
}
