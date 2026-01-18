use macroquad::prelude::{DrawTextureParams, Color, WHITE, draw_texture, draw_line, Texture2D, load_texture};
use rapier2d::prelude::{Collider, ColliderBuilder, vector, nalgebra};
use macroquad_particles::{Emitter, EmitterConfig};
use crate::traits::Drawable;
use crate::camscale::CamScale;

pub struct Engine {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub img: Texture2D,
    pub smoke: Emitter,
}

impl Engine {
    pub async fn new() -> Self {
        let img: Texture2D = load_texture("src/engine.png").await.unwrap();
        let smoke_texture = load_texture("src/steam.png").await.unwrap();

        let smoke = Emitter::new(EmitterConfig {
            emitting: true,
            one_shot: false,
            lifetime: 2.0,
            initial_velocity: 30.0,
            initial_velocity_randomness: 0.8,
            initial_direction_spread: 0.8,
            texture: Some(smoke_texture),
            ..Default::default()
        });

        Self {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            img,
            smoke,
        }
    }

    pub fn make_collider(&self) -> Collider {
        ColliderBuilder::cuboid(self.img.width() / 2.0, self.img.height() / 2.0)
            .translation(vector![0.0, -100.0])
            .friction(0.5)
            .mass(100.0)
            .restitution(0.0)
            .build()
    }

    pub fn set_smoke(&mut self, amt: f32) {
        self.smoke.config.amount = amt as u32;
    }
}

impl Drawable for Engine {
    fn draw(&self) {
        let mut cam_scale = CamScale::new(2.0);
        cam_scale.activate();

        draw_texture(
            &self.img,
            0.0,
            0.0,
            WHITE,
        );

        let wheel_phase = -self.x as f32;

        draw_line(
            15.0 + (wheel_phase).sin() * 3.0,
            25.0 + (wheel_phase).cos() * 3.0,
            35.0,
            25.0,
            1.0,
            Color::from_hex(0x896e2f),
        );

        cam_scale.render(
            self.x,
            self.y,
            Some(DrawTextureParams {
                rotation: self.rotation,
                ..Default::default()
            }));
    }

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}