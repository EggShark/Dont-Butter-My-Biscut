mod player;
mod enemy;
mod collision;

use bottomless_pit::render::Renderer;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::EngineBuilder;
use bottomless_pit::vectors::Vec2;
use enemy::{Enemy, Butter};
use player::Player;


fn main() {
    let mut engine = EngineBuilder::new()
        .set_clear_colour(Colour::Blue)
        .set_window_title("Butter My Biscutt !")
        .set_target_fps(60)
        .unresizable()
        .with_resolution((800, 800))
        .build().unwrap();


    println!("Hello, world!");
    let biscut = Biscut::new(&mut engine);

    engine.run(biscut);
}


struct Biscut {
    player: Player,
    enemy: Enemy,
    butters: Vec<Butter>,
    mouse_pos: Vec2<f32>,
    is_game_over: bool,
}

impl bottomless_pit::Game for Biscut {
    fn render(&self, render_handle: &mut Renderer) {
        self.enemy.draw(render_handle);
        self.player.draw(render_handle);
        self.butters.iter().for_each(|b| b.draw(render_handle));
    }

    fn update(&mut self, engine_handle: &mut Engine) {
        let dt = engine_handle.get_frame_delta_time();
        self.mouse_pos = engine_handle.get_mouse_position();

        self.player.update(engine_handle, dt, &mut self.butters);
        self.enemy.update(dt, &mut self.player, &mut self.butters);

        self.butters.iter_mut().for_each(|s| s.update(dt, &mut self.player));

        let butters = std::mem::take(&mut self.butters);
        self.butters = butters
            .into_iter()
            .filter(|b| b.valid && (b.pos.x > 0.0 && b.pos.x < 800.0) && (b.pos.y > 0.0 && b.pos.y < 800.0))
            .collect::<Vec<Butter>>();
    }
}

impl Biscut {
    fn new(engine_handle: &mut Engine) -> Self {
        let player = Player::new(Vec2{x: 400.0, y: 400.0}, engine_handle);
        let enemy = Enemy::new(Vec2{x: 300.0, y: 100.0}, player.get_center());

        Self {
            player,
            enemy,
            butters: Vec::new(),
            mouse_pos: Vec2{x: 0.0, y: 0.0},
            is_game_over: false,
        }
    }
}

fn move_towards(current: Vec2<f32>, target: Vec2<f32>, max_distance: f32) -> Vec2<f32> {
    let distance_to_player_x = target.x - current.x;
    let distance_to_player_y = target.y - current.y;

    let square_distance = distance_to_player_x.powi(2) + distance_to_player_y.powi(2);


    let total_distance = square_distance.sqrt();

    Vec2 {
        x: current.x + distance_to_player_x/total_distance * max_distance,
        y: current.y + distance_to_player_y/total_distance * max_distance,
    }
}