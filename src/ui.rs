use citro2d::text::TextBuf;
use citro2d::{Color, Frame, RenderTarget};

const TOP_W: f32 = 400.0;
const TOP_H: f32 = 240.0;
pub const BOT_W: f32 = 320.0;
const BOT_H: f32 = 240.0;
const QUAD_W: f32 = TOP_W / 2.0;
const QUAD_H: f32 = TOP_H / 2.0;
const SELECT_BAR_H: f32 = 40.0;
pub const TAB_W: f32 = BOT_W / 4.0;

const P1_DIM: Color = Color::rgb(120, 20, 20);
const P1_BRIGHT: Color = Color::rgb(220, 60, 60);
const P2_DIM: Color = Color::rgb(20, 20, 120);
const P2_BRIGHT: Color = Color::rgb(60, 60, 220);
const P3_DIM: Color = Color::rgb(20, 100, 20);
const P3_BRIGHT: Color = Color::rgb(60, 200, 60);
const P4_DIM: Color = Color::rgb(120, 80, 0);
const P4_BRIGHT: Color = Color::rgb(220, 160, 0);
const WHITE: Color = Color::rgb(255, 255, 255);
const BLACK: Color = Color::rgb(0, 0, 0);
const DARK: Color = Color::rgb(15, 15, 15);
const DARKER: Color = Color::rgb(25, 25, 25);
const DIVIDER: Color = Color::rgb(50, 50, 50);
const BTN_RED: Color = Color::rgb(120, 30, 30);
const BTN_GREEN: Color = Color::rgb(30, 120, 30);
const BTN_ORANGE: Color = Color::rgb(140, 70, 0);
const WARNING: Color = Color::rgb(255, 80, 80);

fn dim(i: usize) -> Color {
    match i {
        0 => P1_DIM,
        1 => P2_DIM,
        2 => P3_DIM,
        3 => P4_DIM,
        _ => DARK,
    }
}

fn bright(i: usize) -> Color {
    match i {
        0 => P1_BRIGHT,
        1 => P2_BRIGHT,
        2 => P3_BRIGHT,
        3 => P4_BRIGHT,
        _ => WHITE,
    }
}

fn name(i: usize) -> &'static str {
    match i {
        0 => "P1",
        1 => "P2",
        2 => "P3",
        3 => "P4",
        _ => "??",
    }
}

fn fmt_i32(n: i32, buf: &mut [u8; 16]) -> &str {
    let mut pos = buf.len();
    let negative = n < 0;
    let mut n = if negative { -(n as i64) } else { n as i64 };
    if n == 0 {
        pos -= 1;
        buf[pos] = b'0';
    } else {
        while n > 0 {
            pos -= 1;
            buf[pos] = b'0' + (n % 10) as u8;
            n /= 10;
        }
    }
    if negative {
        pos -= 1;
        buf[pos] = b'-';
    }
    core::str::from_utf8(&buf[pos..]).unwrap()
}

fn life_x_offset(life: i32) -> f32 {
    if life >= 100 || life <= -100 {
        10.0
    } else if life < 0 || life >= 10 {
        25.0
    } else {
        42.0
    }
}

pub fn draw_top(
    frame: &mut Frame,
    target: &RenderTarget,
    lives: [i32; 4],
    cmd_damage: [[i32; 4]; 4],
    selected: usize,
    show_active: bool,
    text_buf: &TextBuf,
) {
    frame.scene(target, BLACK, |scene| {
        for i in 0..4 {
            let qx = if i % 2 == 0 { 0.0 } else { QUAD_W + 1.0 };
            let qy = if i < 2 { 0.0 } else { QUAD_H + 1.0 };

            let bg = if show_active && i == selected {
                bright(i)
            } else {
                dim(i)
            };
            scene.draw_rect(qx, qy, QUAD_W - 1.0, QUAD_H - 1.0, bg);

            // active accent bar at bottom
            if show_active && i == selected {
                scene.draw_rect(qx, qy + QUAD_H - 5.0, QUAD_W - 1.0, 5.0, WHITE);
            }

            // player name top left
            if let Some(t) = text_buf.parse(name(i)) {
                scene.draw_text(&t, qx + 6.0, qy + 6.0, 0.5, WHITE);
            }

            // life
            let mut buf = [0u8; 16];
            let life_str = fmt_i32(lives[i], &mut buf);
            let lx = qx + life_x_offset(lives[i]);
            let ly = qy + QUAD_H / 2.0 - 30.0;
            if let Some(t) = text_buf.parse(life_str) {
                scene.draw_text(&t, lx, ly, 2.0, WHITE);
            }

            // commander damage row at bottom
            let mut cmd_x = qx + 8.0;
            for from in 0..4 {
                if from == i {
                    continue;
                }
                let dmg = cmd_damage[i][from];
                if dmg > 0 {
                    let clr = if dmg >= 21 { WARNING } else { WHITE };
                    let label = match from {
                        0 => "P1:",
                        1 => "P2:",
                        2 => "P3:",
                        3 => "P4:",
                        _ => "P?:",
                    };
                    let mut buf = [0u8; 16];
                    if let Some(t) = text_buf.parse(label) {
                        scene.draw_text(&t, cmd_x, qy + QUAD_H - 22.0, 0.38, clr);
                    }
                    if let Some(t) = text_buf.parse(fmt_i32(dmg, &mut buf)) {
                        scene.draw_text(&t, cmd_x + 20.0, qy + QUAD_H - 22.0, 0.38, clr);
                    }
                    cmd_x += 42.0;
                }
            }
        }

        // dividers
        scene.draw_rect(QUAD_W - 1.0, 0.0, 2.0, TOP_H, DIVIDER);
        scene.draw_rect(0.0, QUAD_H - 1.0, TOP_W, 2.0, DIVIDER);
    });
}

pub fn draw_bottom(
    frame: &mut Frame,
    target: &RenderTarget,
    selected: usize,
    cmd_damage: [i32; 4],
    text_buf: &TextBuf,
) {
    frame.scene(target, BLACK, |scene| {
        scene.draw_rect(0.0, 0.0, BOT_W, BOT_H, DARK);

        // commander damage section
        let cmd_section_y = 8.0;
        scene.draw_rect(0.0, cmd_section_y, BOT_W, 18.0, DARKER);
        if let Some(t) = text_buf.parse("COMMANDER DAMAGE") {
            scene.draw_text(&t, 85.0, cmd_section_y + 4.0, 0.45, DIVIDER);
        }

        let cmd_btn_y = cmd_section_y + 22.0;
        let cmd_btn_h = 45.0;
        let cmd_btn_w = (BOT_W - 16.0) / 3.0 - 4.0;

        let mut slot = 0;
        for i in 0..4 {
            if i == selected {
                continue;
            }
            let x = 8.0 + slot as f32 * (cmd_btn_w + 4.0);
            let color = dim(i);
            scene.draw_rect(x, cmd_btn_y, cmd_btn_w, cmd_btn_h, color);

            let dmg = cmd_damage[i];
            let dmg_color = if dmg >= 21 { WARNING } else { WHITE };

            if let Some(t) = text_buf.parse(name(i)) {
                scene.draw_text(&t, x + 8.0, cmd_btn_y + 8.0, 0.6, WHITE);
            }

            let mut buf = [0u8; 16];
            if let Some(t) = text_buf.parse(fmt_i32(dmg, &mut buf)) {
                scene.draw_text(
                    &t,
                    x + cmd_btn_w / 2.0 + 5.0,
                    cmd_btn_y + 8.0,
                    0.8,
                    dmg_color,
                );
            }

            slot += 1;
        }

        // ping all button
        let ping_y = cmd_btn_y + cmd_btn_h + 8.0;
        scene.draw_rect(8.0, ping_y, BOT_W - 16.0, 35.0, BTN_ORANGE);
        if let Some(t) = text_buf.parse("PING ALL  -1") {
            scene.draw_text(&t, 100.0, ping_y + 10.0, 0.7, WHITE);
        }

        // life buttons
        let btn_y = ping_y + 35.0 + 8.0;
        let btn_h = 40.0;
        let btn_w = (BOT_W - 20.0) / 4.0 - 4.0;

        let life_btns: [(&str, Color); 4] = [
            ("-10", BTN_RED),
            ("-1", BTN_RED),
            ("+1", BTN_GREEN),
            ("+10", BTN_GREEN),
        ];

        for (idx, (label, color)) in life_btns.iter().enumerate() {
            let x = 8.0 + idx as f32 * (btn_w + 4.0);
            scene.draw_rect(x, btn_y, btn_w, btn_h, *color);
            let text_x = x + btn_w / 2.0 - label.len() as f32 * 5.0;
            if let Some(t) = text_buf.parse(label) {
                scene.draw_text(&t, text_x, btn_y + 12.0, 0.7, WHITE);
            }
        }

        // player select bar
        let bar_y = BOT_H - SELECT_BAR_H;
        for i in 0..4 {
            let x = i as f32 * TAB_W;
            let color = if i == selected { bright(i) } else { dim(i) };
            scene.draw_rect(x, bar_y, TAB_W - 1.0, SELECT_BAR_H, color);
            if let Some(t) = text_buf.parse(name(i)) {
                scene.draw_text(&t, x + TAB_W / 2.0 - 8.0, bar_y + 12.0, 0.7, WHITE);
            }
        }
    });
}
