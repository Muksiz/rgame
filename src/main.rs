//! Rune & Road: the game state (`rgame::app`) rendered as 16×16 sprites on a
//! 480×270 framebuffer and integer-scaled up by this thin Macroquad shell.
//!
//! ```sh
//! cargo run
//! ```

use macroquad::prelude as mq;

use rgame::app::{App, Key, Screen};
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

/// Physical, non-character keys. Letters are read from the OS character stream
/// instead (see the main loop), so the game follows the user's keyboard layout
/// — a Dvorak `h`/`j`/`k`/`l` walks, wherever those keys physically sit.
const KEYMAP: &[(mq::KeyCode, Key)] = &[
    (mq::KeyCode::Up, Key::Up),
    (mq::KeyCode::Down, Key::Down),
    (mq::KeyCode::Left, Key::Left),
    (mq::KeyCode::Right, Key::Right),
    (mq::KeyCode::Enter, Key::Enter),
    (mq::KeyCode::KpEnter, Key::Enter),
    (mq::KeyCode::Escape, Key::Esc),
    (mq::KeyCode::Space, Key::Char(' ')),
    (mq::KeyCode::Backspace, Key::Backspace),
    (mq::KeyCode::PageUp, Key::PageUp),
    (mq::KeyCode::PageDown, Key::PageDown),
];

/// Arrow keys walk the player and so deserve smooth hold-to-repeat. (The vim
/// keys walk too, but repeat via the OS character stream.)
const MOVEMENT: &[mq::KeyCode] = &[
    mq::KeyCode::Up,
    mq::KeyCode::Down,
    mq::KeyCode::Left,
    mq::KeyCode::Right,
];

/// Pick a crisp integer pixel-scale and the framebuffer size that fills a
/// window of `(sw, sh)` edge-to-edge with no letterbox. The scale is chosen so
/// the game keeps roughly its native ~270px logical height; the framebuffer
/// then grows in whichever direction the window is wider, so ultrawide and
/// superultrawide displays simply see more of the world instead of black bars.
fn fit(sw: f32, sh: f32) -> (i32, usize, usize) {
    let scale = (sh / FB_H as f32).round().max(1.0) as i32;
    let fb_w = ((sw / scale as f32).ceil() as usize).max(FB_W);
    let fb_h = ((sh / scale as f32).ceil() as usize).max(FB_H);
    (scale, fb_w, fb_h)
}

#[macroquad::main(conf)]
async fn main() {
    let atlas = Atlas::load();
    let mut fb = Frame::new();
    let mut app = App::new();

    // The texture is recreated whenever the framebuffer's size changes (a
    // window resize), so it always matches the pixels we're pushing.
    let mut image = mq::Image::gen_image_color(fb.w as u16, fb.h as u16, mq::BLACK);
    let mut texture = mq::Texture2D::from_image(&image);
    texture.set_filter(mq::FilterMode::Nearest);
    let mut fb_dims = (fb.w, fb.h);

    let mut tick_acc = 0.0f32;
    let mut held: Option<(mq::KeyCode, f32)> = None;

    while !app.should_quit {
        // Character keys come from the OS text stream, so they respect the
        // active keyboard layout (Dvorak, AZERTY, …) instead of raw QWERTY
        // positions. This drives the vim movement keys, the command letters,
        // and typing a name.
        while let Some(c) = mq::get_char_pressed() {
            if c.is_ascii_alphabetic() || c == '-' || c == '\'' {
                app.on_key(Key::Char(c.to_ascii_lowercase()));
            }
        }
        // Discrete presses of the non-character keys go straight to on_key.
        for &(mk, code) in KEYMAP {
            if mq::is_key_pressed(mk) {
                app.on_key(code);
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
                    app.on_key(code);
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

        // Size the framebuffer to the window so the picture fills it entirely —
        // no black bars, and wide screens reveal more of the world.
        let (sw, sh) = (mq::screen_width(), mq::screen_height());
        let (scale, fb_w, fb_h) = fit(sw, sh);
        fb.resize(fb_w, fb_h);
        if fb_dims != (fb.w, fb.h) {
            image = mq::Image::gen_image_color(fb.w as u16, fb.h as u16, mq::BLACK);
            texture = mq::Texture2D::from_image(&image);
            texture.set_filter(mq::FilterMode::Nearest);
            fb_dims = (fb.w, fb.h);
        }

        gfx::render(&mut fb, &atlas, &app);
        image.bytes.copy_from_slice(&fb.px);
        texture.update(&image);

        // Crisp integer scale, centered — the framebuffer is already sized to
        // cover the window, so any overhang is a symmetric pixel or two.
        let (dw, dh) = (fb.w as f32 * scale as f32, fb.h as f32 * scale as f32);
        mq::clear_background(mq::BLACK);
        mq::draw_texture_ex(
            &texture,
            (sw - dw) / 2.0,
            (sh - dh) / 2.0,
            mq::WHITE,
            mq::DrawTextureParams {
                dest_size: Some(mq::vec2(dw, dh)),
                ..Default::default()
            },
        );
        mq::next_frame().await;
    }
}
