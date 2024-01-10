mod animation;
mod collision;
mod enemy;
mod level;
mod text;
mod player;


use bottomless_pit::input::MouseKey;
use bottomless_pit::engine_handle::Engine;
use bottomless_pit::colour::Colour;
use bottomless_pit::engine_handle::EngineBuilder;
use bottomless_pit::material::{Material, MaterialBuilder};
use bottomless_pit::render::RenderInformation;
use bottomless_pit::texture::Texture;
use bottomless_pit::vectors::Vec2;
use level::Level;
use text::Text;

fn main() {
    let mut engine = EngineBuilder::new()
        .set_clear_colour(Colour::BLUE)
        .set_window_title("Butter My Biscuit!")
        .unresizable()
        .with_resolution((800, 800))
        .build()
        .unwrap();

    let biscut = Biscut::new(&mut engine);

    engine.run(biscut);
}


struct Biscut {
    text: Vec<Text>,
    bg_texture: Material,
    logo: Material,
    plain_mat: Material,
    level: Level,
    state: MainState,
}

impl bottomless_pit::Game for Biscut {
    fn render<'p, 'o>(&'o mut self, mut render_handle: RenderInformation<'p, 'o>) where 'o: 'p {
        self.bg_texture.add_rectangle(Vec2{x: 0.0, y: 0.0}, Vec2{x: 800.0, y: 800.0}, Colour::WHITE, &render_handle);
        self.bg_texture.draw(&mut render_handle);

        match self.state {
            MainState::InGame => {
                self.level.draw(&mut render_handle);
            },
            MainState::EndMenu => {
                self.level.draw(&mut render_handle);
                let r1_pos = self.text[3].pos - Vec2{x: 10.0, y: 10.0};
                let r1_size = Vec2{x: self.text[3].size.x as f32 + 20.0, y: self.text[3].size.y as f32 + 20.0};
                let r2_pos = self.text[4].pos - Vec2{x: 10.0, y: 10.0};
                let r2_size = Vec2{x: self.text[4].size.x as f32 + 20.0, y: self.text[4].size.y as f32 + 20.0};

                self.plain_mat.add_rectangle(r1_pos, r1_size, Colour::WHITE, &render_handle);
                self.plain_mat.add_rectangle(r2_pos, r2_size, Colour::WHITE, &render_handle);
                self.plain_mat.draw(&mut render_handle);
            },
            MainState::MainMenu => {
                let r1_pos = self.text[0].pos - Vec2{x: 10.0, y: 10.0};
                let r1_size = Vec2{x: self.text[0].size.x as f32 + 20.0, y: self.text[0].size.y as f32 + 20.0};
                let r2_pos = self.text[1].pos - Vec2{x: 10.0, y: 10.0};
                let r2_size = Vec2{x: self.text[1].size.x as f32 + 20.0, y: self.text[1].size.y as f32 + 20.0};

                self.logo.add_rectangle(Vec2{x: 250.0, y: 250.0}, Vec2{x: 300.0, y: 300.0}, Colour::WHITE, &render_handle);

                self.plain_mat.add_rectangle(r1_pos, r1_size, Colour::WHITE, &render_handle);
                self.plain_mat.add_rectangle(r2_pos, r2_size, Colour::WHITE, &render_handle);

                self.logo.draw(&mut render_handle);
                self.plain_mat.draw(&mut render_handle);
            },
        }
        self.text.iter_mut().for_each(|t| t.draw(&mut render_handle));
    }

    fn update(&mut self, engine_handle: &mut Engine) {
        let dt = engine_handle.get_frame_delta_time();
        match self.state {
            MainState::InGame => self.in_game_update(engine_handle, dt),
            MainState::MainMenu => self.main_menu_update(engine_handle),
            MainState::EndMenu => self.end_menu_update(engine_handle, dt),
        }
    }
}

impl Biscut {
    fn new(engine_handle: &mut Engine) -> Self {
        let bg_tex = Texture::new(engine_handle, "assets/bg.png");
        let bg_texture = MaterialBuilder::new().add_texture(bg_tex).build(engine_handle);

        let logo_tex = Texture::new(engine_handle, "assets/logo.png");
        let logo = MaterialBuilder::new().add_texture(logo_tex).build(engine_handle);

        let text = vec![
            Text::new("Start Game", 50.0, Vec2{x: 20.0, y: 600.0}, Colour::BLACK, engine_handle),
            Text::new("Quit", 50.0, Vec2{x: 20.0, y: 680.0}, Colour::BLACK, engine_handle),
            Text::new("How to play:", 35.0, Vec2{x: 20.0, y: 20.0}, Colour::BLACK, engine_handle),
            Text::new("W A S D to move", 25.0, Vec2{x: 40.0, y: 60.0}, Colour::BLACK, engine_handle),
            Text::new("Hold left click to charge", 25.0, Vec2{x: 40.0, y: 90.0}, Colour::BLACK, engine_handle),
            Text::new("Release left click to parry incoming butter", 25.0, Vec2{x: 40.0, y: 120.0}, Colour::BLACK, engine_handle),
        ];

        Self {
            level: Level::new(engine_handle),
            logo,
            text,
            bg_texture,
            plain_mat: MaterialBuilder::new().build(engine_handle),
            state: MainState::MainMenu,
        }
    }

    fn in_game_update(&mut self, engine_handle: &mut Engine, dt: f32) {
        self.level.update(engine_handle, dt);
        if self.level.player_dead() {
            self.to_end(engine_handle);
        }
    }

    fn main_menu_update(&mut self, engine_handle: &mut Engine) {
        let mouse_pos = engine_handle.get_mouse_position();
        let mouse_down = engine_handle.is_mouse_key_pressed(MouseKey::Left);

        let r1_pos = self.text[0].pos - Vec2{x: 10.0, y: 10.0};
        let r1_size = Vec2{x: self.text[0].size.x as f32 + 20.0, y: self.text[0].size.y as f32 + 20.0};
        let r2_pos = self.text[1].pos - Vec2{x: 10.0, y: 10.0};
        let r2_size = Vec2{x: self.text[1].size.x as f32 + 20.0, y: self.text[1].size.y as f32 + 20.0};

        if mouse_down && collision::point_in_rect(r2_size, r2_pos, mouse_pos) {
            engine_handle.close();
        }

        if mouse_down && collision::point_in_rect(r1_size, r1_pos, mouse_pos) {
            self.to_game();
        }
    }

    fn end_menu_update(&mut self, engine_handle: &mut Engine, dt: f32) {
        self.level.dead_update(engine_handle, dt);

        let mouse_pos = engine_handle.get_mouse_position();
        let mouse_down = engine_handle.is_mouse_key_pressed(MouseKey::Left);

        let r1_pos = self.text[3].pos - Vec2{x: 10.0, y: 10.0};
        let r1_size = Vec2{x: self.text[3].size.x as f32 + 20.0, y: self.text[3].size.y as f32 + 20.0};
        let r2_pos = self.text[4].pos - Vec2{x: 10.0, y: 10.0};
        let r2_size = Vec2{x: self.text[4].size.x as f32 + 20.0, y: self.text[4].size.y as f32 + 20.0};

        if mouse_down && collision::point_in_rect(r1_size, r1_pos, mouse_pos) {
            engine_handle.close();
        }

        if mouse_down && collision::point_in_rect(r2_size, r2_pos, mouse_pos) {
            self.to_game();
            self.level.restart(engine_handle);
        }
    }

    fn to_game(&mut self) {
        self.text = Vec::new();
        self.state = MainState::InGame;
    }

    fn to_end(&mut self, engine_handle: &mut Engine) {
        self.state = MainState::EndMenu;
        let mut text_1 = Text::new("Congrats!", 40.0, Vec2{x: 400.0, y: 230.0}, Colour::BLACK, engine_handle);
        let mut text_2 = Text::new(&format!("You made it to wave: {}", self.level.get_wave()), 40.0, Vec2{x: 0.0, y: 270.0}, Colour::BLACK, engine_handle);
        let mut text_3 = Text::new(&format!("Succesfully fought {} chefs", self.level.get_kills()), 40.0, Vec2{x: 0.0, y: 310.0}, Colour::BLACK, engine_handle);
        text_1.pos.x = 400.0 - text_1.size.x as f32 / 2.0;
        text_2.pos.x = 400.0 - text_2.size.x as f32 / 2.0;
        text_3.pos.x = 400.0 - text_3.size.x as f32 / 2.0;

        let mut restart = Text::new("Restart", 40.0, Vec2{x: 0.0, y: 390.0}, Colour::BLACK, engine_handle);
        restart.pos.x = 400.0 - restart.size.x as f32 / 2.0;
        let mut quit = Text::new("Quit", 40.0, Vec2{x: 0.0, y: 470.0}, Colour::BLACK, engine_handle);
        quit.pos.x = 400.0 - quit.size.x as f32 / 2.0;

        self.text = vec![text_1, text_2, text_3, restart, quit];
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum MainState {
    MainMenu,
    InGame,
    EndMenu,
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