use std::f32::consts::PI;
use std::time::Instant;

use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::input::{Key, MouseKey};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::texture::Texture;
use bottomless_pit::vectors::Vec2;

use crate::enemy::Butter;
use crate::animation::Anmiation;
use crate::{collision, move_towards};


pub struct Player {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    hp: u8,
    max_hp: u8,
    charge_timer: Option<Instant>,
    weapon_pos: Vec2<f32>,
    weapon_size: Vec2<f32>,
    animation_state: PlayerAnmiationState,
    animations: [Anmiation; 7],
    attack_animations: [Anmiation; 3],
    current_attack_animation: usize,
    full_heart: Material,
    empty_heart: Material,
    rotation: f32,
}

impl Player {
    pub fn new(pos: Vec2<f32>, engine_handle: &mut Engine) -> Self {
        let full_heart_tex = Texture::new(engine_handle, "assets/heart.png");
        let empty_heart_text = Texture::new(engine_handle, "assets/heartEmpty.png");
        let full_heart = MaterialBuilder::new().add_texture(full_heart_tex).build(engine_handle);
        let empty_heart = MaterialBuilder::new().add_texture(empty_heart_text).build(engine_handle);

        Self {
            pos,
            size: Vec2 {x: 50.0, y: 50.0},
            hp: 3,
            max_hp: 3,
            charge_timer: None,
            weapon_pos: Vec2 {x: 0.0, y: 0.0},
            weapon_size: Vec2 {x: 75.0, y: 120.0},
            animation_state: PlayerAnmiationState::IdleDown,
            animations: Self::create_animations(engine_handle),
            attack_animations: Self::create_attack_animations(engine_handle),
            current_attack_animation: 0,
            full_heart,
            empty_heart,
            rotation: 0.0,
        }
    }

    fn create_animations(engine_handle: &mut Engine) -> [Anmiation; 7] {
        [
            Anmiation::new("assets/idleUp.png", Vec2{x: 100.0, y: 100.0}, 4, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/idle.png", Vec2{x: 100.0, y: 100.0}, 4, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/idleSide.png", Vec2{x: 100.0, y: 100.0}, 4, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/walkUp.png", Vec2{x: 100.0, y: 100.0}, 6, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/walk.png", Vec2{x: 100.0, y: 100.0}, 6, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/walkSide.png", Vec2{x: 100.0, y: 100.0}, 6, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/death.png", Vec2{x: 100.0, y: 100.0}, 1, 1.0/6.0, true, engine_handle),
        ]
    }

    fn create_attack_animations(engine_handle: &mut Engine) -> [Anmiation; 3] {
        [
            Anmiation::new("assets/pinIdle.png", Vec2{x: 100.0, y: 160.0}, 1, 100.0, false, engine_handle),
            Anmiation::new("assets/pinCharge.png", Vec2{x: 100.0, y: 160.0}, 3, 1.0/7.0, false, engine_handle),
            Anmiation::new("assets/pinSwing.png", Vec2{x: 100.0, y: 160.0}, 4, 1.0/6.0, false, engine_handle),
        ]
    }

    pub fn draw<'p, 'o>(&'o mut self, render_handle: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        let (index, flipped) = self.animation_state.index();
        self.animations[index].add_instance(render_handle, self.pos, self.size, flipped);

        if !self.is_dead() {
            self.attack_animations[self.current_attack_animation].add_with_rotation(render_handle, self.weapon_pos, self.weapon_size, false, self.rotation);
        }

        let mut offset = 0;
        let step = 75;
        let max = self.max_hp as u32 * step;
        for _ in 0..self.hp {
            self.full_heart.add_rectangle(Vec2{x: offset as f32, y: 0.0}, Vec2{x: 50.0, y: 50.0}, Colour::WHITE, &render_handle);
            offset += step;
        }
        for _ in (offset..max).step_by(step as usize) {
            self.empty_heart.add_rectangle(Vec2{x: offset as f32, y: 0.0}, Vec2{x: 50.0, y: 50.0}, Colour::WHITE, &render_handle);
            offset += step;
        }

        self.animations[index].draw(render_handle);
        self.attack_animations[self.current_attack_animation].draw(render_handle);
        self.empty_heart.draw(render_handle);
        self.full_heart.draw(render_handle);
    }

    pub fn update(&mut self, engine_handle: &mut Engine, dt: f32, butters: &mut Vec<Butter>) {
        if self.is_dead() {
            self.animation_state = PlayerAnmiationState::Dead;
            return;
        }

        let movment_factor = 40.0 * dt;
        let mouse_pos = engine_handle.get_mouse_position();
        let animation_at_start = self.animation_state;
        let attack_animation_start = self.current_attack_animation;
        
        self.weapon_pos = move_towards(self.get_center(), mouse_pos, 40.0);
        self.weapon_pos = self.weapon_pos - Vec2{x: self.size.x/2.0, y: self.size.y/2.0};
        self.rotoate_weapon(mouse_pos);

        // cope freyhoe also 0 = straight up
        let player_dir: u8 = 1 * u8::from(self.rotation > 225.0 && self.rotation <= 315.0) + // down
            2 * u8::from(self.rotation >= 135.0 && self.rotation <= 225.0) + // left
            3 * u8::from((self.rotation > 315.0 && self.rotation <= 360.0) || self.rotation <= 45.0); // right 

        self.animation_state = PlayerAnmiationState::idle_from_dir(player_dir);

        let mut vel = Vec2{x: 0.0, y: 0.0};

        if engine_handle.is_key_down(Key::W) {
            vel.y -= movment_factor;
        }

        if engine_handle.is_key_down(Key::S) {
            vel.y += movment_factor;
        }

        if engine_handle.is_key_down(Key::A) {
            vel.x -= movment_factor;
        }

        if engine_handle.is_key_down(Key::D) {
            vel.x += movment_factor;
        }

        if vel.x != 0.0 || vel.y != 0.0 {
            self.pos = self.pos + vel;
            self.animation_state = PlayerAnmiationState::walking_from_dir(player_dir);
        }

        if self.pos.x > 800.0 - self.size.x {
            self.pos.x = 800.0 - self.size.x;
        } else if self.pos.x < 0.0 {
            self.pos.x = 0.0;
        }

        if self.pos.y > 800.0 - self.size.y {
            self.pos.y = 800.0 - self.size.y
        } else if self.pos.y < 0.0 {
            self.pos.y = 0.0;
        }

        if engine_handle.is_mouse_key_down(MouseKey::Left) && self.attack_animations[self.current_attack_animation].is_done() {
            match self.charge_timer {
                Some(_) => {},
                None => self.charge_timer = Some(Instant::now())
            }
            self.current_attack_animation = 1;
        } else if engine_handle.is_mouse_key_released(MouseKey::Left) {
            match self.charge_timer {
                Some(time) => {
                    let charge_time = time.elapsed().as_secs_f32();

                    if charge_time > 0.2 {
                        self.charge_attack(charge_time, butters, mouse_pos);
                    }
                },
                None => {},
            }

            self.charge_timer = None;
            self.current_attack_animation = 2;
        } else if self.attack_animations[self.current_attack_animation].is_done() {
            self.current_attack_animation = 0;
        }

        if self.animation_state != animation_at_start {
            let (index, _) = animation_at_start.index();
            self.animations[index].reset();
        }

        if attack_animation_start != self.current_attack_animation {
            self.attack_animations[attack_animation_start].reset();
        }


        let (index, _) = animation_at_start.index();
        self.animations[index].update(dt);
        self.attack_animations[self.current_attack_animation].update(dt);
    }

    pub fn restart(&mut self) {
        self.hp = 3;
        self.max_hp = 3;
        self.pos = Vec2{x: 400.0, y: 400.0};
    }

    fn charge_attack(&mut self, charge_time: f32, butters: &mut Vec<Butter>, mouse_pos: Vec2<f32>) {
        // reflect bullets
        butters
            .iter_mut()
            .filter(|b| !b.is_reflected() && collision::rect_rect(b.size, b.pos, self.weapon_size, self.weapon_pos))
            .for_each(|b| {
                b.change_target(mouse_pos, charge_time);
            })
    }

    fn rotoate_weapon(&mut self, mouse_pos: Vec2<f32>) {
        let center = self.get_center();
        let angle = (mouse_pos.y - center.y).atan2(mouse_pos.x - center.x)/PI*180.0;
        self.rotation = (360.0 - angle) % 360.0;
    }

    pub fn get_hit_box(&self) -> (Vec2<f32>, Vec2<f32>) {
        let pos = Vec2{x: self.pos.x, y: self.pos.y + self.size.y/2.0};
        let size = Vec2{x: self.size.x, y: self.size.y/2.0};
        (pos, size)
    }

    pub fn end_wave(&mut self) {
        self.hp += 1;
        if self.hp > self.max_hp {
            self.max_hp += 1;
        }
    }

    pub fn get_center(&self) -> Vec2<f32> {
        Vec2{x: self.pos.x + self.size.x/2.0, y: self.pos.y + self.size.y/2.0}
    }

    pub fn on_hit(&mut self) {
        self.hp = self.hp.saturating_sub(1);
    } 

    pub fn is_dead(&self) -> bool {
        self.hp == 0
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum PlayerAnmiationState {
    IdleUp,
    IdleDown,
    IdleLeft,
    IdleRight,
    WalkingUp,
    WalkingDown,
    WalkingLeft,
    WalkingRight,
    Dead,
}

impl PlayerAnmiationState {
    fn idle_from_dir(dir: u8) -> Self {
        match dir {
            0 => Self::IdleUp,
            1 => Self::IdleDown,
            2 => Self::IdleLeft,
            3 => Self::IdleRight,
            _ => unreachable!(),
        }
    }

    fn walking_from_dir(dir: u8) -> Self {
        match dir {
            0 => Self::WalkingUp,
            1 => Self::WalkingDown,
            2 => Self::WalkingLeft,
            3 => Self::WalkingRight,
            _ => unreachable!(),
        }
    }

    fn index(&self) -> (usize, bool) {
        match self {
            Self::IdleUp => (0, false),
            Self::IdleDown => (1, false),
            Self::IdleLeft => (2, true),
            Self::IdleRight => (2, false),
            Self::WalkingUp => (3, false),
            Self::WalkingDown => (4, false),
            Self::WalkingLeft => (5, true),
            Self::WalkingRight => (5, false),
            Self::Dead => (6, false),
        }
    }
}