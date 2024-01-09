use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::texture::Texture;
use bottomless_pit::vectors::Vec2;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::animation::Anmiation;
use crate::enemy::{Butter, Enemy};
use crate::player::Player;
use crate::text::Text;

pub struct Level {
    player: Player,
    enemies: Vec<Enemy>,
    butters: Vec<Butter>,
    text: Vec<Text>,
    wave_number: u32,
    enemies_spawned: u32,
    spawn_timer: f32,
    butter_texture: Material,
    enemy_animations: [Anmiation; 4],
    random: ThreadRng,
    total_kills: u32,
}

impl Level {
    pub fn new(engine_handle: &mut Engine) -> Self {
        let player = Player::new(Vec2{x: 400.0, y: 400.0}, engine_handle);

        let butter_tex = Texture::new(engine_handle, "assets/butter.png");
        let butter_texture = MaterialBuilder::new().add_texture(butter_tex).build(engine_handle);

        let mut wave_text = Text::new("Wave: 1", 40.0, Vec2{x: 200.0, y: 0.0}, Colour::BLACK, engine_handle);
        let size = wave_text.size;
        let x = 800 - size.x;
        wave_text.pos.x = x as f32;

        let enemy_animations = Enemy::create_animations(engine_handle);

        Self {
            player,
            enemies: Vec::new(),
            text: vec![wave_text],
            butters: Vec::new(),
            wave_number: 1,
            enemies_spawned: 0,
            spawn_timer: 0.0,
            butter_texture,
            enemy_animations,
            random: rand::thread_rng(),
            total_kills: 0,
        }
    }

    pub fn update(&mut self, engine_handle: &mut Engine, dt: f32) {
        self.spawn_enemy(dt);

        self.player.update(engine_handle, dt, &mut self.butters);

        self.enemy_animations.iter_mut().for_each(|a| a.update(dt));
        self.enemies.iter_mut().for_each(|e| e.update(dt, &mut self.player, &mut self.butters, &mut self.random));

        self.butters.iter_mut().for_each(|s| s.update(dt, &mut self.player, &mut self.enemies));
        self.butters.retain(|b| b.valid && (b.pos.x > 0.0 && b.pos.x < 800.0) && (b.pos.y > 0.0 && b.pos.y < 800.0));

        let len_b4 = self.enemies.len() as u32;
        self.enemies.retain(|e| e.is_valid());
        let len_after = self.enemies.len() as u32;

        self.total_kills += len_b4 - len_after;

        if self.player.is_dead() {
            // set target to closet edge
            self.enemies.iter_mut().for_each(|e| e.walk_off());
        }

        if self.is_wave_over() {
            self.set_wave(self.wave_number + 1, engine_handle);
        }
    }

    pub fn dead_update(&mut self, engine_handle: &mut Engine, dt: f32) {
        self.player.update(engine_handle, dt, &mut self.butters);

        self.enemy_animations.iter_mut().for_each(|a| a.update(dt));
        self.butters.iter_mut().for_each(|s| s.update(dt, &mut self.player, &mut self.enemies));
        self.enemies.iter_mut().for_each(|e| e.dead_update(dt, &self.player));
        self.enemies.retain(|e| e.is_valid());
        self.butters.retain(|b| b.valid && (b.pos.x > 0.0 && b.pos.x < 800.0) && (b.pos.y > 0.0 && b.pos.y < 800.0));
    }

    pub fn draw<'p, 'o>(&'o mut self, render_handle: &mut RenderInformation<'p, 'o>) where 'o: 'p {
        // self.enemies.iter().for_each(|b: &'o Enemy| b.draw(render_handle, &mut self.enemy_animations));
        self.enemies.iter().for_each(|e| e.draw(render_handle, &mut self.enemy_animations));
        self.butters.iter().for_each(|b| b.draw(render_handle, &mut self.butter_texture));
        for s in self.enemy_animations.iter_mut() {
            s.draw(render_handle);
        }
        self.butter_texture.draw(render_handle);
        self.player.draw(render_handle);
        self.text.iter_mut().for_each(|t| t.draw(render_handle));
    }

    pub fn restart(&mut self, engine_handle: &mut Engine) {
        self.butters = Vec::new();
        self.enemies = Vec::new();
        self.set_wave(1, engine_handle);
        self.total_kills = 0;
        self.player.restart();
    }

    pub fn player_dead(&self) -> bool {
        self.player.is_dead()
    }

    fn spawn_enemy(&mut self, dt: f32) {
        self.spawn_timer -= dt;

        if self.enemies_spawned < Self::get_enemies_to_spawn(self.wave_number) &&
        self.spawn_timer < 0.0 &&
        self.enemies.len() < self.wave_number as usize + 2
        {
            let side: u8 = self.random.gen_range(0..4);
            let pos: f32 = self.random.gen_range(0.0..800.0);
            if side == 0 {
                self.enemies.push(Enemy::new(Vec2{x: -50.0, y: pos}));
            } else if side == 1 {
                self.enemies.push(Enemy::new(Vec2{x: 850.0, y: pos}));
            } else if side == 2 {
                self.enemies.push(Enemy::new(Vec2{x: pos, y: -50.0}));
            } else if side == 3 {
                self.enemies.push(Enemy::new(Vec2{x: pos, y: 850.0}))
            }
            self.spawn_timer = Self::get_spawn_timer(self.wave_number);
            self.enemies_spawned += 1;
        }
    }

    fn is_wave_over(&self) -> bool {
        self.enemies_spawned == Self::get_enemies_to_spawn(self.wave_number) &&
            self.enemies.len() == 0
    }

    fn set_wave(&mut self, wave: u32, engine_handle: &mut Engine) {

        if wave > self.wave_number {
            self.player.end_wave();
        }

        self.wave_number = wave;
        self.enemies_spawned = 0;
        self.spawn_timer = -1.0;
        self.text[0].change_text(&format!("Wave: {}", self.wave_number), engine_handle);
        self.text[0].pos.x = 800.0 - self.text[0].size.x as f32;
        self.spawn_enemy(0.0);
    }

    pub fn get_wave(&self) -> u32 {
        self.wave_number
    }

    pub fn get_kills(&self) -> u32 {
        self.total_kills
    }

    fn get_enemies_to_spawn(level: u32) -> u32 {
        let level = (level - 1) as f32;
        if level <= 10.0 {
            (f32::powf(level, 1.1).round() as u32 * 5) + 4
        } else {
            f32::powf(level, 1.8) as u32 + 4
        }
    }

    fn get_spawn_timer(wave: u32) -> f32 {
        let wave = (wave-1) as f32;

        f32::max(10.0 - 1.66 * wave, 0.3)
    }
}