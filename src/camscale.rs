use macroquad::prelude::*;
pub struct CamScale {
    pub scale: f32,
    render_target: Option<RenderTarget>,
}

impl CamScale {
    pub fn new(scale: f32) -> Self {
        Self {
            scale,
            render_target: None,
        }
    }

    pub fn activate(&mut self) {
        let width = screen_width() / self.scale;
        let height = screen_height() / self.scale;

        self.render_target = Some(render_target(width as u32, height as u32));

        let camera = Camera2D {
            render_target: self.render_target.clone(),
            zoom: vec2(1.0 / width as f32 * 2.0, 1.0 / height as f32 * 2.0),
            target: vec2(0.0, 0.0),
            ..Default::default()
        };

        set_camera(&camera);
        clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
    }

    pub fn render(&self, x: f32, y: f32, params: Option<DrawTextureParams>) {
        set_default_camera();

        let params = DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..params.unwrap_or_default()
        };

        let render_target = self.render_target.clone().unwrap();

        render_target.texture.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            &render_target.texture,
            x,
            y,
            WHITE,
            params,
        );

    }
}