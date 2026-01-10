use macroquad::prelude::*;
use macroquad::experimental::animation::*;

pub struct Cat {
    pub tail_anim: AnimatedSprite,
    pub feet_anim: AnimatedSprite,
    pub tail_img: Texture2D,
    pub body_img: Texture2D,
    pub feet_img: Texture2D,
}

impl Cat {
    pub async fn new() -> Self {
        let tail_anim = AnimatedSprite::new(
            8,
            8,
            &[Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 8,
                fps: 4,
            }],
            true,
        );
        let feet_anim = AnimatedSprite::new(
            16,
            4,
            &[Animation {
                name: "walk".to_string(),
                row: 0,
                frames: 7,
                fps: 8,
            }],
            true,
        );
        let tail_img = load_texture("src/cat_tail.png").await.unwrap();
        let body_img = load_texture("src/cat_body.png").await.unwrap();
        let feet_img = load_texture("src/cat_feet.png").await.unwrap();
        Self {
            tail_anim,
            feet_anim,
            tail_img,
            body_img,
            feet_img,
        }
    }

    pub fn draw(&mut self, x: f32, y: f32) {
        draw_texture(
            &self.body_img,
            x,
            y,
            WHITE,
        );
        draw_texture_ex(
            &self.tail_img,
            x,
            y + 1.0,
            WHITE,
            DrawTextureParams {
                source: Some(self.tail_anim.frame().source_rect),
                dest_size: Some(self.tail_anim.frame().dest_size),
                ..Default::default()
            },
        );
        draw_texture_ex(
            &self.feet_img,
            x,
            y + 11.0,
            WHITE,
            DrawTextureParams {
                source: Some(self.feet_anim.frame().source_rect),
                dest_size: Some(self.feet_anim.frame().dest_size),
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self) {
        self.tail_anim.update();
        self.feet_anim.update();
    }
}
