//! Rune & Road, the windowed edition: the same `App` as the terminal game,
//! rendered as 16×16 sprites on a 480×270 framebuffer and integer-scaled up.
//!
//! ```sh
//! cargo run --features gfx --bin rune-road-gfx
//! ```

use macroquad::prelude as mq;
use ratatui::crossterm::event::{KeyCode, KeyEvent};

use rgame::app::{App, Screen};
use rgame::gfx::{self, Atlas, FB_H, FB_W, Frame};

const TICK_SECS: f32 = 0.05;
/// Held movement keys repeat after this long, every `REPEAT_EVERY`.
const REPEAT_AFTER: f32 = 0.22;
const REPEAT_EVERY: f32 = 0.09;

fn conf() -> mq::Conf {
    mq::Conf {
        window_title: "Rune & Road".into(),
        window_width: (FB_W * 2) as i32,
        window_height: (FB_H * 2) as i32,
        ..Default::default()
    }
}

const KEYMAP: &[(mq::KeyCode, KeyCode)] = &[
    (mq::KeyCode::Up, KeyCode::Up),
    (mq::KeyCode::Down, KeyCode::Down),
    (mq::KeyCode::Left, KeyCode::Left),
    (mq::KeyCode::Right, KeyCode::Right),
    (mq::KeyCode::Enter, KeyCode::Enter),
    (mq::KeyCode::Escape, KeyCode::Esc),
    (mq::KeyCode::Space, KeyCode::Char(' ')),
    (mq::KeyCode::PageUp, KeyCode::PageUp),
    (mq::KeyCode::PageDown, KeyCode::PageDown),
    (mq::KeyCode::W, KeyCode::Char('w')),
    (mq::KeyCode::A, KeyCode::Char('a')),
    (mq::KeyCode::S, KeyCode::Char('s')),
    (mq::KeyCode::D, KeyCode::Char('d')),
    (mq::KeyCode::H, KeyCode::Char('h')),
    (mq::KeyCode::J, KeyCode::Char('j')),
    (mq::KeyCode::K, KeyCode::Char('k')),
    (mq::KeyCode::L, KeyCode::Char('l')),
    (mq::KeyCode::E, KeyCode::Char('e')),
    (mq::KeyCode::C, KeyCode::Char('c')),
    (mq::KeyCode::Q, KeyCode::Char('q')),
    (mq::KeyCode::F, KeyCode::Char('f')),
];

/// Keys that walk the player and so deserve hold-to-repeat.
const MOVEMENT: &[mq::KeyCode] = &[
    mq::KeyCode::Up,
    mq::KeyCode::Down,
    mq::KeyCode::Left,
    mq::KeyCode::Right,
    mq::KeyCode::W,
    mq::KeyCode::A,
    mq::KeyCode::S,
    mq::KeyCode::D,
];

#[macroquad::main(conf)]
async fn main() {
    let atlas = Atlas::load();
    let mut fb = Frame::new();
    let mut app = App::new();

    let mut image = mq::Image::gen_image_color(FB_W as u16, FB_H as u16, mq::BLACK);
    let texture = mq::Texture2D::from_image(&image);
    texture.set_filter(mq::FilterMode::Nearest);

    let mut tick_acc = 0.0f32;
    let mut held: Option<(mq::KeyCode, f32)> = None;

    while !app.should_quit {
        // Discrete presses go straight to the same on_key the TUI uses.
        for &(mk, code) in KEYMAP {
            if mq::is_key_pressed(mk) {
                app.on_key(KeyEvent::from(code));
                if MOVEMENT.contains(&mk) {
                    held = Some((mk, 0.0));
                }
            }
        }
        // Hold-to-walk: repeat the movement key while it stays down.
        if let Some((mk, ref mut t)) = held {
            if mq::is_key_down(mk) && matches!(app.screen, Screen::World) {
                *t += mq::get_frame_time();
                if *t >= REPEAT_AFTER {
                    *t -= REPEAT_EVERY;
                    let code = KEYMAP.iter().find(|(m, _)| *m == mk).unwrap().1;
                    app.on_key(KeyEvent::from(code));
                }
            } else {
                held = None;
            }
        }

        tick_acc += mq::get_frame_time();
        while tick_acc >= TICK_SECS {
            tick_acc -= TICK_SECS;
            app.on_tick();
        }

        gfx::render(&mut fb, &atlas, &app);
        image.bytes.copy_from_slice(&fb.px);
        texture.update(&image);

        // Integer scale, centered — crisp pixels, no smearing.
        let scale = ((mq::screen_width() / FB_W as f32).floor())
            .min((mq::screen_height() / FB_H as f32).floor())
            .max(1.0);
        let (dw, dh) = (FB_W as f32 * scale, FB_H as f32 * scale);
        mq::clear_background(mq::BLACK);
        mq::draw_texture_ex(
            &texture,
            (mq::screen_width() - dw) / 2.0,
            (mq::screen_height() - dh) / 2.0,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::vec2(dw, dh)),
                ..Default::default()
            },
        );
        mq::next_frame().await;
    }
}
