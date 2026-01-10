use std::iter::repeat_with;
use macroquad::prelude::{Color, YELLOW, draw_line};
use crate::traits::Drawable;

#[derive(Default)]
pub struct GrassBlade {
    pub phase: f32,
    pub x: f32,
    pub y: f32,

    stiff: f32,
    height: f32,
    color: Color,
}

pub trait ColorFunction {
    fn float_to_color(&self, perc: f32) -> Color;
}

#[derive(Default)]
pub struct GrassBuilder {
    pub count: usize,
    pub x: (f32, f32),
    pub y: (f32, f32),
    pub height: (f32, f32),
    pub stiff: (f32, f32),
    pub color: Box<dyn ColorFunction>,
}

struct DefaultColorFunction;

impl ColorFunction for DefaultColorFunction {
    fn float_to_color(&self, perc: f32) -> Color {
        let mut color = YELLOW;
        color.g = 0.7 + perc * 0.3;
        color
    }
}

impl Default for Box<dyn ColorFunction> {
    fn default() -> Self {
        Box::new(DefaultColorFunction)
    }
}

#[inline]
pub fn rand_f32(min: f32, max: f32) -> f32 {
    fastrand::f32() * (max - min) + min
}

impl GrassBlade {
    fn rand(builder: &GrassBuilder) -> Self {
        GrassBlade {
            x: rand_f32(builder.x.0, builder.x.1),
            y: rand_f32(builder.y.0, builder.y.1),
            height: rand_f32(builder.height.0, builder.height.1),
            stiff: rand_f32(builder.stiff.0, builder.stiff.1),
            color: builder.color.float_to_color(fastrand::f32()),
            ..Default::default()
        }
    }
}

impl Drawable for GrassBlade {
    fn draw(&self) {
        let xoff: f32 = (self.phase + self.x).sin() * (1.0-self.stiff)*3.0;
        draw_line(self.x, self.y, self.x + xoff, self.y - self.height, 2.0, self.color);
    }

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl GrassBuilder {
    pub fn build(&self) -> Vec<GrassBlade> {
        repeat_with(|| GrassBlade::rand(self))
            .take(self.count)
            .collect()
    }
}