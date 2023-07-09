use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::vectors::Vec2;
use bottomless_pit::render::Renderer;

pub struct Text {
    text: String,
    scale: f32,
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    colour: Colour
}

impl Text {
    pub fn new(text: &str, scale: f32, pos: Vec2<f32>, colour: Colour, engine: &mut Engine) -> Self {
        let size = engine.measure_text(text, scale);

        Self {
            text: text.to_string(),
            scale,
            pos,
            size,
            colour,
        }
    }

    pub fn draw(&self, render_handle: &mut Renderer) {
        render_handle.draw_text(&self.text, self.pos, self.scale, self.colour);
    }

    pub fn change_text(&mut self, new_text: &str, engine: &mut Engine) {
        self.size = engine.measure_text(new_text, self.scale);
        self.text = new_text.to_string();
    }
}