use macroquad::prelude::*;

pub trait Drawable {
    fn get_position(&self) -> (f32, f32);
    fn draw(&self);
}