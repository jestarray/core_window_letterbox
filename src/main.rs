use raylib::ffi::TextureFilterMode::FILTER_BILINEAR;
use raylib::prelude::*;
use std::convert::TryInto;

const GAME_WIDTH: u32 = 640;
const GAME_HEIGHT: u32 = 480;
// 20 x 15 IN TILES
fn main() {
    let mut cam = Camera2D {
        offset: Vector2::new(0.0, 0.0),
        target: Vector2::default(),
        rotation: 0.0,
        zoom: 1.0,
    };
    let (mut rl, thread) = raylib::init()
        .size(GAME_WIDTH as i32, GAME_HEIGHT as i32)
        .title("example")
        .resizable()
        .vsync()
        .build();

    //pre-generate static parts of the tilemap like floors
    let mut render_texture_generation_map = rl
        .load_render_texture(&thread, GAME_WIDTH, GAME_HEIGHT)
        .expect("could not create texture");
    render_texture_generation_map
        .texture_mut()
        .set_texture_filter(FILTER_BILINEAR);

    //24x24 TILES
    let floor_image = Image::load_image("assets/floors.png").expect("could not load floor image");
    let floor_texture = rl
        .load_texture_from_image(&thread, &floor_image)
        .expect("could not convert floor image to texture");
    println!(
        "fllor texture size: {} x {}",
        floor_texture.width, floor_texture.height
    );

    rl.set_target_fps(30);

    //START PRE GENERATE THE RENDER TEXTURE BECAUSE IT IS STATIC.
    /* {
        let mut d = rl.begin_drawing(&thread);
        let mut d = d.begin_texture_mode(&mut render_texture_generation_map);
        let mut d = d.begin_mode_2D(cam);

        //PRETEND THIS IS ITERATING THROUGH A TILED TMX MAP AND PARSING DATA
        //even though we try to draw the entire tilemap(24x24), the render texture is (20x15) , so drawing cuts off at 20x15
        /*     for y in 0..23 {
            for x in 0..23 {
                //DRAW TILE BY TILE
                d.draw_texture_rec(
                    &floor_texture,
                    Rectangle::new((x * 32) as f32, (y * 32) as f32, 32.0, 32.0),
                    Vector2::new((x * 32) as f32, (y * 32) as f32),
                    Color::WHITE,
                );
            }
        } */
        dbg!(floor_texture.width);
        dbg!(floor_texture.height);
        d.draw_texture_rec(
            &floor_texture,
            Rectangle::new(
                0.0,
                0.0,
                floor_texture.width as f32,
                floor_texture.height as f32,
            ),
            Vector2::new(0.0, 0.0),
            Color::WHITE,
        );
    } */

    let mut speed = 5.0;
    while !rl.window_should_close() {
        let scale = (rl.get_screen_width() as f32 / GAME_WIDTH as f32)
            .min(rl.get_screen_height() as f32 / GAME_HEIGHT as f32);
        let right = rl.is_key_down(KeyboardKey::KEY_D);
        let down = rl.is_key_down(KeyboardKey::KEY_S);
        let left = rl.is_key_down(KeyboardKey::KEY_A);
        let up = rl.is_key_down(KeyboardKey::KEY_W);

        let mut d = rl.begin_drawing(&thread);

        if right {
            cam.offset.x -= speed;
        }
        if down {
            cam.offset.y -= speed;
        }
        if left {
            cam.offset.x += speed;
        }
        if up {
            cam.offset.y += speed;
        }

        d.clear_background(Color::PINK);
        {
            let mut d = d.begin_texture_mode(&mut render_texture_generation_map);
            //BEGIN CAMERA ALLOWS TO DRAW FLOOR TEXTURE FULLY INTO RENDER_TEXTURE_GENERATION_MAP? EVEN THOUGH THE SIZE OF GENERATION MAP IS MUCH SMALLER?
            let mut d = d.begin_mode_2D(&cam);
            //d.clear_background(Color::PINK);
            //is this drawing a texture into the camera or the render texture? because floor_texture is much bigger than render_texture
            d.draw_texture_rec(
                &floor_texture,
                Rectangle::new(
                    0.0,
                    0.0,
                    floor_texture.width as f32,
                    floor_texture.height as f32,
                ),
                Vector2::new(0.0, 0.0),
                Color::WHITE,
            );
        }
        //scale the 20x15 texture
        d.draw_texture_pro(
            render_texture_generation_map.texture(),
            Rectangle::new(
                -cam.offset.x,
                cam.offset.y,
                render_texture_generation_map.texture.width as f32,
                -render_texture_generation_map.texture.height as f32,
            ),
            Rectangle::new(
                (d.get_screen_width() as f32 - (GAME_WIDTH as f32 * scale as f32)) * 0.5,
                (d.get_screen_height() as f32 - (GAME_HEIGHT as f32 * scale)) * 0.5,
                GAME_WIDTH as f32 * scale,
                GAME_HEIGHT as f32 * scale,
            ),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );
    }
}
