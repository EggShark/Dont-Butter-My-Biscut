use bottomless_pit::engine_handle::Engine;
use bottomless_pit::render::Renderer;
use bottomless_pit::texture::TextureIndex;
use bottomless_pit::vectors::Vec2;

pub struct Anmiation {
    sprite_sheet: TextureIndex,
    sprite_size: Vec2<f32>,
    frames: usize,
    current_frame: usize,
    frame_time_counter: f32,
    frame_time: f32,
    looping: bool,
}

impl Anmiation {
    pub fn new(texture_path: &str, sprite_size: Vec2<f32>, frames: usize, frame_time: f32, looping: bool, engine_handle: &mut Engine) -> Self {
        let texture = engine_handle.create_texture(texture_path).unwrap();


        Self {
            sprite_sheet: texture,
            sprite_size,
            frames,
            current_frame: 0,
            frame_time_counter: 0.0,
            frame_time,
            looping,
        }
    }

    pub fn draw(&self, render_handle: &mut Renderer, draw_pos: Vec2<f32>, draw_size: Vec2<f32>, flipped: bool) {
        let dir = if flipped {
            -1.0
        } else {
            1.0
        };

        render_handle.draw_textured_rectangle_with_uv(
            draw_pos,
            draw_size.x,
            draw_size.y,
            &self.sprite_sheet,
            Vec2{x: self.current_frame as f32 * self.sprite_size.x, y: 0.0},
            Vec2{x: self.sprite_size.x * dir, y: self.sprite_size.y},
        )
    }

    pub fn draw_with_rotation(&self, render_handle: &mut Renderer, draw_pos: Vec2<f32>, draw_size: Vec2<f32>, flipped: bool, deg: f32) {
        let dir = if flipped {
            -1.0
        } else {
            1.0
        };

        render_handle.draw_textured_rectangle_ex(
            draw_pos,
            draw_size.x,
            draw_size.y,
            &self.sprite_sheet,
            deg,
            Vec2{x: self.current_frame as f32 * self.sprite_size.x, y: 0.0},
            Vec2{x: self.sprite_size.x * dir, y: self.sprite_size.y},
        );
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