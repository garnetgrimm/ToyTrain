use macroquad::prelude::*;
use rapier2d::prelude::*;
use macroquad_particles::{Emitter, EmitterConfig};

mod cat;
mod traits;
mod grass;

use cat::Cat;
use grass::{GrassBuilder, GrassBlade};
use traits::Drawable;


pub struct Sprite {
    texture: Texture2D,
    handle: RigidBodyHandle,
}

pub struct Engine {
    sprite: Sprite,
    smoke: Emitter,
}

const INTERNAL_WIDTH: u32 = 160;
const INTERNAL_HEIGHT: u32 = 75;

impl Sprite {
    async fn load(path: &str, position: Vec2, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) -> Sprite {
        let texture = load_texture(path).await.expect("Failed to load texture");

        let collider = ColliderBuilder::cuboid(texture.width() / 2.0, texture.height() / 2.0)
            .friction(0.5)
            .mass(100.0)
            .restitution(0.0)
            .build();

        let body = RigidBodyBuilder::dynamic() 
            .translation(vector![position.x, position.y])
            .build();

        let handle = rigid_body_set.insert(body);

        collider_set.insert_with_parent(collider, handle, rigid_body_set);

        Sprite {
            handle: handle,
            texture: texture
        }
    }
}

fn draw_engine(engine: &mut Engine, body: &RigidBody) {
    let train_y = body.translation().y.round();
    let train_x = body.translation().x.round();

    engine.smoke.draw(vec2(train_x + 45.0, train_y));

    draw_texture_ex(
        &engine.sprite.texture,
        train_x,
        train_y,
        WHITE,
        DrawTextureParams {
            rotation: body.rotation().angle(),
            ..Default::default()
        }
    );

    let wheel_phase = -body.translation().x as f32;

    draw_line(
        train_x + 15.0 + (wheel_phase).sin() * 3.0,
        train_y + 25.0 + (wheel_phase).cos() * 3.0,
        train_x + 35.0,
        train_y + 25.0,
        1.0,
        Color::from_hex(0x896e2f),
    );
}

fn draw_car(car: &Sprite, body: &RigidBody) {
    draw_texture_ex(
        &car.texture,
        body.translation().x.round(),
        body.translation().y.round(),
        WHITE,
        DrawTextureParams {
            rotation: body.angvel(),
            ..Default::default()
        }
    );
}

#[macroquad::main("Simple Sprite Example")]
async fn main() {    

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();


    let mut cat = Cat::new().await;

    let grass_builder = GrassBuilder {
        count: 32,
        ..Default::default()
    };
    let mut grass: Vec<GrassBlade> = grass_builder.build();

    let ground_collider = ColliderBuilder::cuboid(100.0, 10.0) // half-extents
        .friction(0.5)
        .translation(vector![0.0, 0.0])
        .build();
    ground_collider.translation().x;
    collider_set.insert(ground_collider);

    let engine_sprite = Sprite::load(
        "src/engine.png",
        vec2(0.0, -20.0),
        &mut rigid_body_set,
        &mut collider_set,
    ).await;

    let smoke_texture = load_texture("src/steam.png").await.expect("Failed to load particle texture");

    let track_texture = load_texture("src/tracks.png").await.expect("Failed to load track texture");

    let smoke_emitter = Emitter::new(EmitterConfig {
        emitting: true,
        one_shot: false,
        lifetime: 2.0,
        initial_velocity: 30.0,
        initial_velocity_randomness: 0.8,
        initial_direction_spread: 0.8,
        texture: Some(smoke_texture),
        ..Default::default()
    });

    let mut engine = Engine {
        sprite: engine_sprite,
        smoke: smoke_emitter,
    };

    let mut cars = Vec::<Sprite>::new();

    for i in 0..3 {
        cars.push(Sprite::load(
            "src/car.png",
            vec2(90.0 + i as f32 * 70.0, 30.0),
            &mut rigid_body_set,
            &mut collider_set,
        ).await);
    }

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

    let mut camera = Camera2D {
        render_target: Some(render_target.clone()),
        zoom: vec2(1.0 / INTERNAL_WIDTH as f32 * 2.0, 1.0 / INTERNAL_HEIGHT as f32 * 2.0),
        target: vec2(INTERNAL_WIDTH as f32 / 2.0, INTERNAL_HEIGHT as f32 / 2.0),
        ..Default::default()
    };


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


        set_camera(&camera);

        let mut impulse_vector = vector![0.0, 0.0];
        if is_key_down(KeyCode::Right) {
            impulse_vector.x += 100.0;
        }
        if is_key_down(KeyCode::Left) {
            impulse_vector.x -= 100.0;
        }

        {
            let engine_body = rigid_body_set.get_mut(engine.sprite.handle).unwrap();
            engine_body.apply_impulse(impulse_vector, true);
            engine.smoke.config.amount = engine_body.linvel().x.abs() as u32 / 5;
        }


        clear_background(Color::from_hex(0x896e2f));

        let engine_body = &rigid_body_set[engine.sprite.handle];
        let num_sky_shades: i32 = 5;
        for i in 0..num_sky_shades {
            let mut color = Color::from_hex(0x27c1e9);
            let perc = i as f32 / num_sky_shades as f32;
            color.r += perc;
            draw_rectangle(
                engine_body.translation().x - INTERNAL_WIDTH as f32 / 2.0,
                0.0 - perc * (INTERNAL_HEIGHT as f32),
                INTERNAL_WIDTH as f32,
                20.0,
                color,
            );
        }

        for blade in &mut grass {
            if blade.x + INTERNAL_WIDTH as f32 / 2.0 < engine_body.translation().x {
                blade.x += INTERNAL_WIDTH as f32;
            }
            if blade.x - INTERNAL_WIDTH as f32 / 2.0 > engine_body.translation().x {
                blade.x -= INTERNAL_WIDTH as f32;
            }
        }

        camera.target.x = engine_body.translation().x.round();
        camera.target.y = engine_body.translation().y.round();

        cat.x = engine_body.translation().x;
        cat.y = engine_body.translation().y;

        for i in 0..10 {
            draw_texture(&track_texture, (i*32) as f32, 0.0, WHITE);
        }

        draw_engine(&mut engine, engine_body);

        for car in &cars {
            draw_car(&car, &rigid_body_set[car.handle]);
        }


        let mut items_to_draw = Vec::<&dyn Drawable>::new();
        grass.iter().for_each(|blade| items_to_draw.push(blade));
        items_to_draw.push(&cat);
        items_to_draw.sort_by(|a, b| a.get_position().1.partial_cmp(&b.get_position().1).unwrap());

        items_to_draw.iter().for_each(|item| item.draw());

        cat.update();

        set_default_camera();
        
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
