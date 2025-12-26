use macroquad::prelude::*;
use rapier2d::prelude::*;

struct Train {
    engine: Texture2D,
    car: Texture2D,
    x: f32,
    y: f32,
}

const INTERNAL_WIDTH: u32 = 320;
const INTERNAL_HEIGHT: u32 = 150;

#[macroquad::main("Simple Sprite Example")]
async fn main() {    
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground (a static cuboid). */
    let ground_collider = ColliderBuilder::cuboid(10.0, 0.1) // half-extents
        .friction(0.8)
        .build();
    collider_set.insert(ground_collider);

    /* Create a bouncing ball (a dynamic rigid body with a ball collider). */
    let ball_body = RigidBodyBuilder::dynamic() // makes the body dynamic (simulated by physics)
        .translation(vector![0.0, 10.0]) // initial position
        .build();
    let ball_collider = ColliderBuilder::ball(0.5) // radius
        .restitution(0.7) // bounciness (0.0 to 1.0)
        .build();
    let ball_body_handle = rigid_body_set.insert(ball_body);
    collider_set.insert_with_parent(ball_collider, ball_body_handle, &mut rigid_body_set);

    /* Create other structures for the physics pipeline. */
    let gravity = vector![0.0, -9.81];
    let mut physics_pipeline = PhysicsPipeline::new();
    let integration_parameters = IntegrationParameters::default();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    let font = load_ttf_font_from_bytes(include_bytes!("Pixel-Western.ttf")).unwrap();

    let render_target = render_target(INTERNAL_WIDTH, INTERNAL_HEIGHT);
    render_target.texture.set_filter(FilterMode::Nearest);

    let camera = Camera2D {
        render_target: Some(render_target.clone()),
        // Adjust the zoom to match your desired camera behavior within the low-res space
        zoom: vec2(1.0 / INTERNAL_WIDTH as f32 * 2.0, 1.0 / INTERNAL_HEIGHT as f32 * 2.0),
        target: vec2(0.0, 0.0), // Center the camera if needed
        ..Default::default()
    };

    let engine = load_texture("src/engine.png").await.expect("Failed to load engine");
    let car = load_texture("src/car.png").await.expect("Failed to load car");

    let mut train = Train { engine, car, x: 0.0, y: 0.0};


    loop {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        // Get and print the ball's position
        let ball_body = &mut rigid_body_set[ball_body_handle];

        set_camera(&camera);

        if is_key_down(KeyCode::Right) {
            let impulse_vector = vector![-0.1, 0.0];
            ball_body.apply_impulse(impulse_vector, true);
        }

        if is_key_down(KeyCode::Left) {
            let impulse_vector = vector![0.1, 0.0];
            ball_body.apply_impulse(impulse_vector, true);
        }


        train.y = (-ball_body.translation().y * 10.0).round();
        train.x = (-ball_body.translation().x * 10.0).round();

        clear_background(BLUE); // Clear screen

        draw_texture(&train.engine, train.x, train.y, WHITE);
        for i in 0..3 {
            draw_texture(&train.car, train.x - 50.0 - 55.0*(i as f32), train.y, WHITE);
        }

        let wheel_phase = ball_body.translation().x as f32 * get_time() as f32 / 3.0;

        draw_line(
            train.x + 20.0 + (wheel_phase).sin() * 3.0,
            train.y + 25.0 + (wheel_phase).cos() * 3.0,
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

        draw_text_ex(
            format!("Choo Choo <3").as_str(),
            0.0,
            60.0,
            TextParams {
                font: Some(&font),
                font_size: 30,
                color: Color::from_hex(0x896e2f),
                ..Default::default()
            },
        );

        next_frame().await // Wait for next frame
    }
}
