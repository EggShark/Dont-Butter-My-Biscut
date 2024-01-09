use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::text::TextMaterial;
use bottomless_pit::vectors::Vec2;
use bottomless_pit::render::RenderInformation;

pub struct Text {
    text: TextMaterial,
    pub pos: Vec2<f32>,
    pub size: Vec2<u32>,
    colour: Colour
}

impl Text {
    pub fn new(text: &str, scale: f32, pos: Vec2<f32>, colour: Colour, engine: &mut Engine) -> Self {
        println!("text: {}", text);
        let text_material = TextMaterial::new(text, colour, scale, scale * 1.2, engine);
        let size = text_material.get_measurements();

        Self {
            text: text_material,
            pos,
            size,
            colour,
        }
    }

    pub fn draw<'p, 'o>(&'o mut self, render_handle: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        self.text.add_instance(self.pos, Colour::WHITE, &render_handle);

        self.text.draw(render_handle);
    }

    pub fn change_text(&mut self, new_text: &str, engine: &mut Engine) {
        self.text.set_text(new_text, self.colour, engine);
        self.text.prepare(engine);
    }
}