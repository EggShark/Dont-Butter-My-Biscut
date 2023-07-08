use std::f32::consts::PI;
use std::time::Instant;

use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::input::{Key, MouseKey};
use bottomless_pit::render::Renderer;
use bottomless_pit::texture::TextureIndex;
use bottomless_pit::vectors::Vec2;

use crate::enemy::Butter;
use crate::{collision, move_towards};


pub struct Player {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    colour: Colour,
    hp: u8,
    charge_timer: Option<Instant>,
    weapon_pos: Vec2<f32>,
    weapon_size: Vec2<f32>,
    texture: TextureIndex,
    rotation: f32,
}

impl Player {
    pub fn new(pos: Vec2<f32>, engine_handle: &mut Engine) -> Self {
        let texture = engine_handle.create_texture("assets/arrow.png").unwrap();
        Self {
            pos,
            size: Vec2 {x: 50.0, y: 50.0},
            colour: Colour::Green,
            hp: 3,
            charge_timer: None,
            weapon_pos: Vec2 {x: 0.0, y: 0.0},
            weapon_size: Vec2 {x: 50.0, y: 50.0},
            texture,
            rotation: 0.0,
        }
    }

    pub fn draw(&self, render_handle: &mut Renderer) {
        render_handle.draw_rectangle(self.pos, self.size.x, self.size.y, self.colour);
        render_handle.draw_textured_rect_with_rotation(self.weapon_pos, self.weapon_size.x, self.weapon_size.y, &self.texture, self.rotation);
    }

    pub fn update(&mut self, engine_handle: &mut Engine, dt: f32, butters: &mut Vec<Butter>) {
        let movment_factor = 40.0 * dt;

        let mouse_pos = engine_handle.get_mouse_position();
        self.weapon_pos = move_towards(self.pos, mouse_pos, 40.0);
        self.weapon_pos = self.weapon_pos + Vec2{x: 10.0, y: 10.0};
        self.rotoate_weapon(mouse_pos);
        println!("{}", self.rotation);

        if engine_handle.is_key_down(Key::W) {
            self.pos.y -= movment_factor;
        }

        if engine_handle.is_key_down(Key::S) {
            self.pos.y += movment_factor;
        }

        if engine_handle.is_key_down(Key::A) {
            self.pos.x -= movment_factor;
        }

        if engine_handle.is_key_down(Key::D) {
            self.pos.x += movment_factor;
        }

        if engine_handle.is_mouse_key_down(MouseKey::Left) {
            match self.charge_timer {
                Some(_) => {},
                None => self.charge_timer = Some(Instant::now())
            }
        } else if engine_handle.is_mouse_key_released(MouseKey::Left) {
            match self.charge_timer {
                Some(time) => {
                    let charge_time = time.elapsed().as_secs_f32();
                    println!("attack was charged for: {}", charge_time);

                    if charge_time > 0.3 {
                        self.charge_attack(charge_time, butters, mouse_pos);
                    } else {
                        self.regular_attack(charge_time);
                    }
                },
                None => unreachable!(),
            }

            self.charge_timer = None;
        }
    }

    fn charge_attack(&mut self, charge_time: f32, butters: &mut Vec<Butter>, mouse_pos: Vec2<f32>) {
        // reflect bullets
        butters
            .iter_mut()
            .filter(|b| !b.is_reflected() && collision::rect_rect(b.size, b.pos, self.weapon_size, self.weapon_pos))
            .for_each(|b| {
                b.change_target(mouse_pos);
            })
    }

    fn rotoate_weapon(&mut self, mouse_pos: Vec2<f32>) {
        let center = self.get_center();
        let angle = (mouse_pos.y - center.y).atan2(mouse_pos.x - center.x)/PI*180.0;
        self.rotation = (360.0 - angle) % 360.0;
    }

    fn regular_attack(&mut self, charge_time: f32) {
        // contact damage
    }

    pub fn get_center(&self) -> Vec2<f32> {
        Vec2{x: self.pos.x + 25.0, y: self.pos.y + 25.0}
    }

    pub fn on_hit(&mut self) {
        self.hp = self.hp.saturating_sub(1);
        println!("hit");
    } 

    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
}