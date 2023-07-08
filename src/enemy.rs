use std::time::Instant;

use bottomless_pit::colour::Colour;
use bottomless_pit::render::Renderer;
use bottomless_pit::vectors::Vec2;

use crate::{collision, move_towards};
use crate::player::Player;


pub struct Enemy {
    pub pos: Vec2<f32>,
    size: Vec2<f32>,
    colour: Colour,
    shot_cooldown: Instant,
}

impl Enemy {
    pub fn new(pos: Vec2<f32>, player_pos: Vec2<f32>) -> Self {
        Self {
            pos,
            size: Vec2{x: 50.0, y: 50.0},
            colour: Colour::Red,
            shot_cooldown: Instant::now(),
        }
    }

    pub fn draw(&self, render_handle: &mut Renderer) {
        render_handle.draw_rectangle(self.pos, self.size.x, self.size.y, self.colour);
    }

    pub fn update(&mut self, dt: f32, player: &mut Player, butters: &mut Vec<Butter>) {
        if self.shot_cooldown.elapsed().as_secs_f32() > 1.0 {
            butters.push(Butter::new(self.pos, player.get_center()));
            self.shot_cooldown = Instant::now();
        }
    }
}

pub struct Butter {
    velocity: Vec2<f32>,
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    colour: Colour,
    reflected: bool,
    pub valid: bool,
}

impl Butter {
    pub fn new(starting_pos: Vec2<f32>, target: Vec2<f32>) -> Self {
        let move_towards = move_towards(starting_pos, target, 44.0);
        let diff = starting_pos - move_towards;
        Self {
            pos: starting_pos,
            size: Vec2{x: 15.0, y: 15.0},
            velocity: diff,
            colour: Colour::Black,
            reflected: false,
            valid: true,
        }
    }

    pub fn update(&mut self, dt: f32, player: &mut Player) {
        let new_x = self.pos.x - (self.velocity.x * dt);
        let new_y = self.pos.y - (self.velocity.y * dt);

        self.pos = Vec2{x: new_x, y: new_y};

        if self.reflected {

        } else {
            let hit = collision::rect_rect(self.size, self.pos, player.size, player.pos);
            if hit {
                player.on_hit();
                self.valid = false;
            }
        }
    }

    pub fn draw(&self, render_handle: &mut Renderer) {
        render_handle.draw_rectangle(self.pos, self.size.x, self.size.y, self.colour);
    }

    pub fn change_target(&mut self, new_target: Vec2<f32>) {
        let move_towards = move_towards(self.pos, new_target, 44.0);
        let diff = self.pos - move_towards;
        self.velocity = diff;
        self.reflected = true;
    }

    pub fn is_reflected(&self) -> bool {
        self.reflected
    }
}