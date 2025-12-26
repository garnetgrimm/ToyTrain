use macroquad::prelude::*;

struct Train {
    texture: Texture2D,
    x: f32,
    y: f32,
}

const INTERNAL_WIDTH: u32 = 320;
const INTERNAL_HEIGHT: u32 = 150;

#[macroquad::main("Simple Sprite Example")]
async fn main() {
    let render_target = render_target(INTERNAL_WIDTH, INTERNAL_HEIGHT);
    render_target.texture.set_filter(FilterMode::Nearest);

    let camera = Camera2D {
        render_target: Some(render_target.clone()),
        // Adjust the zoom to match your desired camera behavior within the low-res space
        zoom: vec2(1.0 / INTERNAL_WIDTH as f32 * 2.0, 1.0 / INTERNAL_HEIGHT as f32 * 2.0),
        target: vec2(0.0, 0.0), // Center the camera if needed
        ..Default::default()
    };

    let texture = load_texture("src/engine.png").await.expect("Failed to load texture");

    let mut train = Train { texture, x: 0.0, y: 0.0};


    loop {
        set_camera(&camera);

        if is_key_down(KeyCode::Right) { train.x += 2.0; }
        if is_key_down(KeyCode::Left) { train.x -= 2.0; }
        if is_key_down(KeyCode::Up) { train.y -= 2.0; }
        if is_key_down(KeyCode::Down) { train.y += 2.0; }

        clear_background(BLUE); // Clear screen

        draw_texture(&train.texture, train.x, train.y, WHITE);
        draw_texture(&train.texture, train.x, train.y, WHITE);

        draw_line(
            train.x + 20.0 + (get_time() as f32 * 10.0).sin() * 3.0,
            train.y + 25.0 + (get_time() as f32 * 10.0).cos() * 3.0,
            train.x + 40.0,
            train.y + 25.0,
            1.0,
            Color::from_hex(0x896e2f),
        );

        // Switch back to the default camera (draws to the full screen)
        set_default_camera();
        
        // Clear the actual screen
        clear_background(BLUE);

        // Draw the low-resolution render target texture onto the full screen, scaled up.
        // Because its filter mode is Nearest, it will look pixelated.
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        next_frame().await // Wait for next frame
    }
}
