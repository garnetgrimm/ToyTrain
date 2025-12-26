use macroquad::prelude::*;
use rapier2d::prelude::*;
use macroquad_particles::{Emitter, EmitterConfig};

pub struct Sprite {
    texture: Texture2D,
    body: RigidBody,
    collider: Collider,
}

const INTERNAL_WIDTH: u32 = 320;
const INTERNAL_HEIGHT: u32 = 150;

async fn load_train(path: &str) -> Sprite {
    let texture = load_texture(path).await.expect("Failed to load engine");

    let collider = ColliderBuilder::cuboid(texture.width(), texture.height())
        .friction(0.5)
        .mass(100.0)
        .restitution(0.1) // bounciness (0.0 to 1.0)
        .build();

    let body = RigidBodyBuilder::dynamic() 
        .translation(vector![0.0, -texture.height() * 2.0])
        .build();

    Sprite {
        body: body,
        collider: collider,
        texture: texture
    }
}

#[macroquad::main("Simple Sprite Example")]
async fn main() {    

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground (a static cuboid). */
    let ground_collider = ColliderBuilder::cuboid(INTERNAL_WIDTH as f32 / 3.0, 1.0) // half-extents
        .friction(0.5)
        .build();
    collider_set.insert(ground_collider);

    let engine = load_train("src/engine.png").await;
    let engine_handle = rigid_body_set.insert(engine.body);
    collider_set.insert_with_parent(engine.collider, engine_handle, &mut rigid_body_set);

    /* Create other structures for the physics pipeline. */
    let gravity = vector![0.0, 9.81];
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

    let particle_texture = load_texture("src/steam.png").await.expect("Failed to load particle texture");

    let mut emitter = Emitter::new(EmitterConfig {
        emitting: true,
        one_shot: false,
        lifetime: 2.0,
        initial_velocity: 30.0,
        initial_velocity_randomness: 0.8,
        initial_direction_spread: 0.8,
        texture: Some(particle_texture),
        ..Default::default()
    });

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

        let engine_body = &mut rigid_body_set[engine_handle];

        set_camera(&camera);

        if is_key_down(KeyCode::Right) {
            let impulse_vector = vector![100.0, 0.0];
            engine_body.apply_impulse(impulse_vector, true);
        }

        if is_key_down(KeyCode::Left) {
            let impulse_vector = vector![-100.0, 0.0];
            engine_body.apply_impulse(impulse_vector, true);
        }

        let train_y = engine_body.translation().y.round();
        let train_x = engine_body.translation().x.round();

        clear_background(BLUE); // Clear screen

        draw_rectangle(INTERNAL_WIDTH as f32 / -3.0, -5.0, INTERNAL_WIDTH as f32 / 1.5, 10.0, RED);

        emitter.draw(vec2(train_x + 45.0, train_y));

        draw_texture_ex(&
            engine.texture,
            train_x,
            train_y,
            WHITE,
            DrawTextureParams {
                rotation: engine_body.angvel(),
                ..Default::default()
            }
        );

        let wheel_phase = -engine_body.translation().x as f32;

        draw_line(
            train_x + 20.0 + (wheel_phase).sin() * 3.0,
            train_y + 25.0 + (wheel_phase).cos() * 3.0,
            train_x + 40.0,
            train_y + 25.0,
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
