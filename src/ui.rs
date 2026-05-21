use citro2d::text::TextBuf;
use citro2d::{Color, Frame, RenderTarget};

const TOP_HALF: f32 = 200.0;
const HEIGHT: f32 = 240.0;
const BOT_WIDTH: f32 = 320.0;
const BOT_HALF_W: f32 = BOT_WIDTH / 2.0;
const BOT_HALF_H: f32 = 120.0;

const P1_BG: Color = Color::rgb(26, 0, 0);
const P1_ACTIVE: Color = Color::rgb(139, 0, 0);
const P1_ACCENT: Color = Color::rgb(255, 68, 68);
const P1_BTN_HI: Color = Color::rgb(80, 20, 20);
const P1_BTN_LO: Color = Color::rgb(50, 10, 10);
const P2_BG: Color = Color::rgb(0, 0, 26);
const P2_ACTIVE: Color = Color::rgb(0, 0, 139);
const P2_ACCENT: Color = Color::rgb(68, 68, 255);
const P2_BTN_HI: Color = Color::rgb(20, 20, 80);
const P2_BTN_LO: Color = Color::rgb(10, 10, 50);
const WHITE: Color = Color::rgb(255, 255, 255);
const DIM: Color = Color::rgb(170, 170, 170);
const BLACK: Color = Color::rgb(0, 0, 0);
const DIVIDER: Color = Color::rgb(51, 51, 51);

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

pub fn draw_top(
    frame: &mut Frame,
    target: &RenderTarget,
    p1_life: i32,
    p2_life: i32,
    selected: usize,
    show_active: bool,
    text_buf: &TextBuf,
) {
    frame.scene(target, BLACK, |scene| {
        let p1_color = if !show_active || selected == 0 {
            P1_ACTIVE
        } else {
            P1_BG
        };
        scene.draw_rect(0.0, 0.0, TOP_HALF - 1.0, HEIGHT, p1_color);

        let p2_color = if !show_active || selected == 1 {
            P2_ACTIVE
        } else {
            P2_BG
        };
        scene.draw_rect(TOP_HALF + 1.0, 0.0, TOP_HALF - 1.0, HEIGHT, p2_color);

        scene.draw_rect(TOP_HALF - 1.0, 0.0, 2.0, HEIGHT, DIVIDER);

        if show_active {
            if selected == 0 {
                scene.draw_rect(0.0, HEIGHT - 6.0, TOP_HALF - 1.0, 6.0, P1_ACCENT);
            } else {
                scene.draw_rect(TOP_HALF + 1.0, HEIGHT - 6.0, TOP_HALF - 1.0, 6.0, P2_ACCENT);
            }
        }

        if let Some(t) = text_buf.parse("PLAYER 1") {
            scene.draw_text(&t, 10.0, 15.0, 0.5, DIM);
        }

        let mut buf = [0u8; 16];
        if let Some(t) = text_buf.parse(fmt_i32(p1_life, &mut buf)) {
            scene.draw_text(&t, 40.0, 85.0, 3.0, WHITE);
        }

        if let Some(t) = text_buf.parse("PLAYER 2") {
            scene.draw_text(&t, 210.0, 15.0, 0.5, DIM);
        }

        let mut buf = [0u8; 16];
        if let Some(t) = text_buf.parse(fmt_i32(p2_life, &mut buf)) {
            scene.draw_text(&t, 240.0, 85.0, 3.0, WHITE);
        }
    });
}

pub fn draw_bottom(frame: &mut Frame, target: &RenderTarget, text_buf: &TextBuf) {
    frame.scene(target, BLACK, |scene| {
        // P1 + (top left)
        scene.draw_rect(0.0, 0.0, BOT_HALF_W, BOT_HALF_H, P1_BTN_HI);
        // P1 - (bottom left)
        scene.draw_rect(0.0, BOT_HALF_H, BOT_HALF_W, BOT_HALF_H, P1_BTN_LO);
        // P2 + (top right)
        scene.draw_rect(BOT_HALF_W, 0.0, BOT_HALF_W, BOT_HALF_H, P2_BTN_HI);
        // P2 - (bottom right)
        scene.draw_rect(BOT_HALF_W, BOT_HALF_H, BOT_HALF_W, BOT_HALF_H, P2_BTN_LO);

        // dividers
        scene.draw_line(BOT_HALF_W, 0.0, BOT_HALF_W, HEIGHT, 2.0, DIVIDER);
        scene.draw_line(0.0, BOT_HALF_H, BOT_WIDTH, BOT_HALF_H, 2.0, DIVIDER);

        // labels
        if let Some(t) = text_buf.parse("P1  +") {
            scene.draw_text(&t, 55.0, 50.0, 1.0, WHITE);
        }
        if let Some(t) = text_buf.parse("P1  -") {
            scene.draw_text(&t, 55.0, 170.0, 1.0, WHITE);
        }
        if let Some(t) = text_buf.parse("P2  +") {
            scene.draw_text(&t, 215.0, 50.0, 1.0, WHITE);
        }
        if let Some(t) = text_buf.parse("P2  -") {
            scene.draw_text(&t, 215.0, 170.0, 1.0, WHITE);
        }
    });
}
