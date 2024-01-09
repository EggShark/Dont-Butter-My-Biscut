use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::texture::Texture;
use bottomless_pit::vectors::Vec2;

pub struct Anmiation {
    sprite_sheet: Material,
    sprite_size: Vec2<f32>,
    frames: usize,
    current_frame: usize,
    frame_time_counter: f32,
    frame_time: f32,
    looping: bool,
}

impl Anmiation {
    pub fn new(texture_path: &str, sprite_size: Vec2<f32>, frames: usize, frame_time: f32, looping: bool, engine: &mut Engine) -> Self {
        // let texture = engine_handle.create_texture(texture_path).unwrap();
        let texture = Texture::new(engine, texture_path);
        let sprite_sheet = MaterialBuilder::new()
            .add_texture(texture)
            .build(engine);

        Self {
            sprite_sheet,
            sprite_size,
            frames,
            current_frame: 0,
            frame_time_counter: 0.0,
            frame_time,
            looping,
        }
    }

    pub fn add_instance(&mut self, render_handle: &RenderInformation, draw_pos: Vec2<f32>, draw_size: Vec2<f32>, flipped: bool) {
        let dir = if flipped {
            -1.0
        } else {
            1.0
        };

        self.sprite_sheet.add_rectangle_with_uv(
            draw_pos,
            draw_size,
            Vec2{x: self.current_frame as f32 * self.sprite_size.x, y: 0.0},
            Vec2{x: self.sprite_size.x * dir, y: self.sprite_size.y},
            Colour::WHITE,
            render_handle
        );
    }

    pub fn add_with_rotation(&mut self, render_handle: &mut RenderInformation, draw_pos: Vec2<f32>, draw_size: Vec2<f32>, flipped: bool, deg: f32) {
        let dir = if flipped {
            -1.0
        } else {
            1.0
        };

        self.sprite_sheet.add_rectangle_ex(
            draw_pos,
            draw_size,
            Colour::WHITE,
            deg,
            Vec2{x: self.current_frame as f32 * self.sprite_size.x, y: 0.0},
            Vec2{x: self.sprite_size.x * dir, y: self.sprite_size.y},
            render_handle
        );
    }

    pub fn draw<'p, 'o>(&'o mut self, render_handle: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        self.sprite_sheet.draw(render_handle);
    }

    pub fn update(&mut self, dt: f32) {
        self.frame_time_counter += dt;

        if self.frame_time_counter > self.frame_time {
            if self.looping {
                self.current_frame = (self.current_frame + 1) % self.frames;
            } else if self.current_frame < self.frames - 1 {
                self.current_frame += 1;
            }
        }

        self.frame_time_counter = self.frame_time_counter % self.frame_time;
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.frame_time_counter = 0.0;
    }

    pub fn is_done(&self) -> bool {
        !self.looping && self.current_frame == self.frames - 1
    }
}