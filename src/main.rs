use raylib::ffi::ConfigFlag::FLAG_VSYNC_HINT;
use raylib::ffi::ConfigFlag::FLAG_WINDOW_RESIZABLE;
use raylib::ffi::EndDrawing;
use raylib::ffi::EndTextureMode;
use raylib::ffi::GetScreenHeight;
use raylib::ffi::GetScreenWidth;
use raylib::ffi::SetConfigFlags;
use raylib::ffi::TextureFilterMode::FILTER_BILINEAR;
use raylib::prelude::*;
use std::convert::TryInto;

const GAME_WIDTH: u32 = 640;
const GAME_HEIGHT: u32 = 480;
fn main() {
    let window_width = 800;
    let window_height = 450;
    unsafe {
        SetConfigFlags(FLAG_WINDOW_RESIZABLE as u8 | FLAG_VSYNC_HINT as u8);
    }
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("example")
        .build();
    rl.set_window_min_size(320, 240);

    // Render texture initialization, used to hold the rendering result so we can easily resize it

    let mut target = rl
        .load_render_texture(&thread, GAME_WIDTH, GAME_HEIGHT)
        .expect("could not create texture");
    target.texture_mut().set_texture_filter(FILTER_BILINEAR);

    let mut colors = [Color::new(0, 0, 0, 255); 10];
    for c in colors.iter_mut() {
        c.r = get_random_value(100, 250).try_into().unwrap();
        c.g = get_random_value(100, 250).try_into().unwrap();
        c.b = get_random_value(100, 250).try_into().unwrap();
    }

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let scale = (rl.get_screen_width() as f32 / GAME_WIDTH as f32)
            .min(rl.get_screen_height() as f32 / GAME_HEIGHT as f32);
        let mut d = rl.begin_drawing(&thread);
        d.begin_texture_mode(&mut target);
        d.clear_background(Color::WHITE);

        for i in 0..10 {
            d.draw_rectangle(
                0,
                (GAME_HEIGHT as i32 / 10) * i as i32,
                GAME_WIDTH as i32,
                GAME_HEIGHT as i32 / 10,
                colors[i],
            );
        }

        d.draw_text(
            "If executed inside a window,\nyou can resize the window,\nand see the screen scaling!",
            10,
            25,
            20,
            Color::BLACK,
        );
        unsafe {
            EndTextureMode();
        }

        unsafe {
            d.draw_texture_pro(
                target.texture(),
                Rectangle::new(
                    0.0,
                    0.0,
                    target.texture.width as f32,
                    -target.texture.height as f32,
                ),
                Rectangle::new(
                    (GetScreenWidth() as f32 - (GAME_WIDTH as f32 * scale as f32)) * 0.5,
                    (GetScreenHeight() as f32 - (GAME_HEIGHT as f32 * scale)) * 0.5,
                    GAME_WIDTH as f32 * scale,
                    GAME_HEIGHT as f32 * scale,
                ),
                Vector2::new(0.0, 0.0),
                0.0,
                Color::WHITE,
            );
        }

        /*     d.draw_texture_pro(
            target.texture(),
            Rectangle::new(
                0.0,
                0.0,
                target.texture().width() as f32,
                -target.texture().height() as f32,
            ),
            Rectangle::new(
                (d.get_screen_width() as f32 - GAME_WIDTH as f32 * scale) * 0.5,
                (d.get_screen_height() as f32 - GAME_HEIGHT as f32 * scale) * 0.5,
                GAME_WIDTH as f32 * scale,
                GAME_HEIGHT as f32 * scale,
            ),
            Vector2::default(),
            0.0,
            Color::WHITE,
        ); */

        unsafe {
            EndDrawing();
        }
    }
}
