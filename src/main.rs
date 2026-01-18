use macroquad::prelude::*;
use ::rand::rand_core::le;
use rapier2d::prelude::*;

mod camscale;
mod cat;
mod traits;
mod grass;
mod engine;
mod track;

use cat::Cat;
use grass::{GrassBuilder, GrassBlade};
use track::Track;
use traits::Drawable;

const INTERNAL_WIDTH: u32 = 160;
const INTERNAL_HEIGHT: u32 = 75;

#[macroquad::main("Simple Sprite Example")]
async fn main() {    

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let grass_builder = GrassBuilder {
        count: 32,
        ..Default::default()
    };
    let mut grass: Vec<GrassBlade> = grass_builder.build();
    let mut cat = Cat::new().await;
    let mut engine = engine::Engine::new().await;
    let tracks = track::Track::new(3).await;

    let engine_collider = engine.make_collider();
    let engine_body = RigidBodyBuilder::dynamic().translation(vector![0.0, -45.0]).build();
    let engine_handle = rigid_body_set.insert(engine_body);
    collider_set.insert_with_parent(engine_collider, engine_handle, &mut rigid_body_set);

    let ground_collider = tracks.make_collider();
    collider_set.insert(ground_collider);

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

        let mut impulse_vector = vector![0.0, 0.0];
        if is_key_down(KeyCode::Right) {
            impulse_vector.x += 100.0;
        }
        if is_key_down(KeyCode::Left) {
            impulse_vector.x -= 100.0;
        }

        {
            let engine_body = rigid_body_set.get_mut(engine_handle).unwrap();
            engine_body.apply_impulse(impulse_vector, true);
            engine.set_smoke(engine_body.linvel().x.abs() as f32 / 5.0);
        }

        let engine_body = &rigid_body_set[engine_handle];

        for blade in &mut grass {
            if blade.x + INTERNAL_WIDTH as f32 / 2.0 < engine_body.translation().x {
                blade.x += INTERNAL_WIDTH as f32;
            }
            if blade.x - INTERNAL_WIDTH as f32 / 2.0 > engine_body.translation().x {
                blade.x -= INTERNAL_WIDTH as f32;
            }
        }

        let render_target = render_target(INTERNAL_WIDTH, INTERNAL_HEIGHT);
        render_target.texture.set_filter(FilterMode::Nearest);

        let mut camera = Camera2D {
            render_target: Some(render_target.clone()),
            zoom: vec2(1.0 / INTERNAL_WIDTH as f32 * 2.0, 1.0 / INTERNAL_HEIGHT as f32 * 2.0),
            target: vec2(INTERNAL_WIDTH as f32 / 2.0, INTERNAL_HEIGHT as f32 / 2.0),
            ..Default::default()
        };

        set_camera(&camera);

        camera.target.x = engine_body.translation().x.round() + engine.img.width() / 2.0;
        camera.target.y = engine_body.translation().y.round();

        engine.x = engine_body.translation().x;
        engine.y = engine_body.translation().y;
        engine.rotation = engine_body.rotation().angle();

        let mut items_to_draw = Vec::<&dyn Drawable>::new();
        grass.iter().for_each(|blade| items_to_draw.push(blade));
        items_to_draw.push(&cat);
        // items_to_draw.push(&engine);
        items_to_draw.sort_by(|a, b| a.get_position().1.partial_cmp(&b.get_position().1).unwrap());

        items_to_draw.iter().for_each(|item| item.draw());
        engine.smoke.draw(vec2(engine.x + 45.0, engine.y));

        cat.update();

        set_default_camera();
        
        clear_background(Color::from_hex(0x896e2f));

        let num_sky_shades: i32 = 5;
        let sky_height = screen_height() / 2.0;
        for i in 0..num_sky_shades {
            let mut color = Color::from_hex(0x27c1e9);
            let perc = i as f32 / num_sky_shades as f32;
            color.r += perc;
            draw_rectangle(
                0.0,
                perc * sky_height,
                screen_width(),
                sky_height / num_sky_shades as f32,
                color,
            );
        }

        tracks.draw();
        engine.draw();
        
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
            "< Choo Choo <",
            230.0,
            75.0,
            TextParams {
                font_scale: 1.0 + (get_time() * 10.0).sin() as f32 * (0.001*engine_body.linvel().x.abs()),
                font: Some(&font),
                font_size: 30,
                color: BLACK,
                ..Default::default()
            },
        );

        next_frame().await // Wait for next frame
    }
}
