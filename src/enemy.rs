use web_time::Instant;

use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::material::Material;
use bottomless_pit::render::RenderInformation;
use bottomless_pit::vectors::Vec2;
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::animation::Anmiation;
use crate::{collision, move_towards};
use crate::player::Player;


pub struct Enemy {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    shot_cooldown: Instant,
    valid: bool,
    speed: f32,
    desision_timer: f32,
    target_pos: Vec2<f32>,
    hp: f32,
    current_animation: usize,
}

impl Enemy {
    pub fn new(pos: Vec2<f32>) -> Self {
        Self {
            pos,
            size: Vec2{x: 50.0, y: 50.0},
            shot_cooldown: Instant::now(),
            valid: true,
            speed: 40.0,
            desision_timer: 100.0,
            target_pos: Vec2{x: 0.0, y: 0.0},
            hp: 65.0,
            current_animation: 0,
        }
    }

    pub fn create_animations(engine_handle: &mut Engine) -> [Anmiation; 4] {
        [
            Anmiation::new("assets/chefWalk.png", Vec2{x: 170.0, y: 170.0}, 5, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/chefWalkBack.png", Vec2{x: 170.0, y: 170.0}, 5, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/throwFront.png", Vec2{x: 170.0, y: 170.0}, 5, 1.0/6.0, true, engine_handle),
            Anmiation::new("assets/throwBack.png", Vec2{x: 170.0, y: 170.0}, 5, 1.0/6.0, true, engine_handle),
        ]
    }

    pub fn draw(&self, render_handle: &mut RenderInformation, animations: &mut [Anmiation]) {
        animations[self.current_animation].add_instance(render_handle, self.pos, self.size, false);
    }

    pub fn update(&mut self, dt: f32, player: &Player, butters: &mut Vec<Butter>, rand: &mut ThreadRng) {
        if self.shot_cooldown.elapsed().as_secs_f32() > 2.0 {
            butters.push(Butter::new(self.get_center(), player.get_center()));
            self.shot_cooldown = Instant::now();
        }

        self.desision_timer += dt;
        if self.desision_timer > 1.5 {
            self.switch_target(player, rand);
            self.desision_timer %= 1.5;
        }

        self.pos = move_towards(self.pos, self.target_pos, self.speed * dt);
        
        if self.pos == self.target_pos {
            self.desision_timer = 0.0;
            self.switch_target(player, rand);
        }
        
        if self.shot_cooldown.elapsed().as_secs_f32() > 1.5 {
            self.current_animation = 2;
        } else {
            self.current_animation = 0;
        }

        if player.get_center().y < self.pos.y {
            self.current_animation += 1;
        }
    }

    pub fn dead_update(&mut self, dt: f32, player: &Player) {
        self.pos = move_towards(self.pos, self.target_pos, self.speed * dt);

        self.current_animation = 0;
        if player.get_center().y > self.pos.y {
            self.current_animation += 1;
        }

        self.valid = self.pos.x > -50.0 && self.pos.x < 850.0 && self.pos.y > -50.0 && self.pos.y < 850.0;
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn on_hit(&mut self, damage: f32) {
        self.hp -= damage;
        if self.hp < 0.0 {
            self.valid = false;
        }
    }

    pub fn get_center(&self) -> Vec2<f32> {
        Vec2{x: self.pos.x + self.size.x/2.0, y: self.pos.y + self.size.y/2.0}
    }

    pub fn walk_off(&mut self) {
        let left_dist = self.pos.x + 50.0;
        let right_dist = 850.0 - self.pos.x;
        let top_dist = self.pos.y + 50.0;
        let bottom_dist = 850.0 - self.pos.y;
    
        let clostest_x = left_dist.min(right_dist);
        let closest_y = top_dist.min(bottom_dist);

        if clostest_x < closest_y {
            if left_dist < right_dist {
                self.target_pos = Vec2{x: -50.0, y: self.pos.y};
            } else {
                self.target_pos = Vec2{x: 850.0, y: self.pos.y};
            }
        } else {
            if top_dist < bottom_dist {
                self.target_pos = Vec2{x: self.pos.x, y: -50.0};
            } else {
                self.target_pos = Vec2{x: self.pos.x, y: 850.0};
            }
        }
    }

    fn switch_target(&mut self, player: &Player, rand: &mut ThreadRng) {
        // very sophisticated AI
        let rng = rand.gen::<f32>();
        if rng <= 0.333 {
            self.target_pos = player.get_center();
        } else if rng <= 0.666 {
            let player_pos = player.get_center();
            let x: f32 = (rand.gen_range(0.0..300.0) * (-1.0 * u8::from(rand.gen_range::<u8, _>(1..3) == 1) as f32)) + player_pos.x;
            let y = (300.0 - x * (-1.0 * u8::from(rand.gen_range::<u8, _>(1..3) == 1) as f32)) + player_pos.y;
            self.target_pos = Vec2{x, y};
        } else {
            let x: f32 = rand.gen_range(0.0..800.0);
            let y: f32 = rand.gen_range(0.0..800.0);
            self.target_pos = Vec2{x, y};
        }
    }
}

pub struct Butter {
    velocity: Vec2<f32>,
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    reflected: bool,
    damage: f32,
    pub valid: bool,
}

impl Butter {
    pub fn new(starting_pos: Vec2<f32>, target: Vec2<f32>) -> Self {
        let move_towards = move_towards(starting_pos, target, 100.0);
        let diff = starting_pos - move_towards;
        Self {
            pos: starting_pos,
            size: Vec2{x: 15.0, y: 15.0},
            velocity: diff,
            reflected: false,
            damage: 30.0,
            valid: true,
        }
    }

    pub fn update(&mut self, dt: f32, player: &mut Player, enemies: &mut [Enemy]) {
        let new_x = self.pos.x - (self.velocity.x * dt);
        let new_y = self.pos.y - (self.velocity.y * dt);

        self.pos = Vec2{x: new_x, y: new_y};
        let (p_box_pos, p_box_size) = player.get_hit_box();

        let hit_player = collision::rect_rect(self.size, self.pos, p_box_size, p_box_pos);

        if self.reflected {
            enemies
                .iter_mut()
                .filter(|e| collision::rect_rect(self.size, self.pos, e.size, e.pos))
                .for_each(|e| {
                    e.on_hit(self.damage);
                    self.valid = false;
                });
        }

        if hit_player {
            player.on_hit();
            self.valid = false;
        }
    }

    pub fn draw(&self, render_handle: &mut RenderInformation, butter_material: &mut Material) {
        butter_material.add_rectangle(self.pos, self.size, Colour::WHITE, &render_handle);
        // need to draw it but later.....
    }

    pub fn change_target(&mut self, new_target: Vec2<f32>, charge_time: f32) {
        let move_towards = move_towards(self.pos, new_target, 100.0);
        let diff = self.pos - move_towards;
        self.velocity = diff;
        self.damage += 33.0 * (charge_time + 0.7).log10() + 10.0;
        self.reflected = true;
    }

    pub fn is_reflected(&self) -> bool {
        self.reflected
    }
}