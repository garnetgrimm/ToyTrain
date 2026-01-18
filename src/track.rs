use macroquad::prelude::{ WHITE, draw_texture, Texture2D, load_texture};
use rapier2d::prelude::{Collider, ColliderBuilder, vector, nalgebra};
use crate::traits::Drawable;
use crate::camscale::CamScale;

pub struct Track {
    pub segments: u32,
    pub img: Texture2D,
}

impl Track {
    pub async fn new(segments: u32) -> Self {
        Self {
            img: load_texture("src/tracks.png").await.unwrap(),
            segments,
        }
    }

    pub fn make_collider(&self) -> Collider {
        let track_width = self.img.width() * self.segments as f32;
        ColliderBuilder::cuboid(track_width / 2.0, 100.0)
            .friction(0.5)
            .translation(vector![track_width / 2.0, 0.0])
            .build()
    }
}

impl Drawable for Track {
    fn draw(&self) {
        let mut cam_scale = CamScale::new(2.0);
        cam_scale.activate();

        for i in 0..self.segments {
            draw_texture(
                &self.img,
                self.img.width() as f32 * i as f32,
                0.0,
                WHITE,
            );
        }

        cam_scale.render(0.0, 0.0, None);
    }

    fn get_position(&self) -> (f32, f32) {
        (0.0, 0.0)
    }
}