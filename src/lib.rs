use geom::Direction;
use graphics::color::BLACK;
use models::{GameObject, enemy};
use models::bullet::Bullet;
use opengl_graphics::{GlyphCache, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use models::player::Player;
use models::enemy::Enemy;
use rand::{thread_rng, Rng};
use find_folder;

use crate::config::font::draw_text;

mod geom;
pub mod config;
pub mod models;

const SHOOT_COOLDOWN: u32 = 50;

enum GameStatus {
    Normal,
    Win,
    Died
}
pub struct App<'a>{
    pub window: config::GraphicsConfig, // OpenGL drawing backend.
    glyph_cache: GlyphCache<'a>,
    player: Player,
    game_status: GameStatus,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    enemy_spawn_columns: Vec<f64>,
    fire_bullet: bool,
    score: u32,
    ammo: u32,
    shoot_cooldown: u32,
    uptime: u64,
}

impl<'a> App<'a> {
    pub fn new(window: config::GraphicsConfig) -> App<'a> {
        let size = window.size;

        let (x, y) = (f64::from(size.width / 2.0),
                      f64::from(size.height / 2.0));

        let player = Player::new(x, y);

        // Load font(s) used in the game.
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        let glyph_cache = GlyphCache::new(assets.join("fonts/PxPlus_IBM_VGA8.ttf"), (), TextureSettings::new())
        .expect("Unable to load font");

        let mut i = size.width / 10.0;
        let mut enemy_spawn_columns: Vec<f64> = Vec::new();
        while i < size.width
        {
            enemy_spawn_columns.push(i);
            i += size.width / 10.0;
        }

        App {
            glyph_cache,
            window,
            player,
            game_status: GameStatus::Normal,
            enemies: Vec::new(),
            bullets: Vec::new(),
            enemy_spawn_columns,
            fire_bullet: false,
            shoot_cooldown: SHOOT_COOLDOWN,
            ammo: 20,
            score: 0,
            uptime: 0,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.window.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            match self.game_status {
                GameStatus::Normal => {
                    self.player.render(&c, gl);
                    for enemy in self.enemies.iter() {
                        enemy.render(&c, gl);
                    }
        
                    for bullet in self.bullets.iter() {
                        bullet.render(&c, gl);
                    }
        
                    let curr_score = format!("Score: {:?}", self.score);
                    draw_text(curr_score.as_str(), [12.0, 24.0], 12, &mut self.glyph_cache, &c, gl);
        
                    let curr_score = format!("Health: {:?}", self.player.health);
                    draw_text(curr_score.as_str(), [self.window.size.width - 125.0, 24.0], 12, &mut self.glyph_cache, &c, gl);


                    let ammo = format!("Bullets: {:?}", self.ammo);
                    draw_text(ammo.as_str(), [self.window.size.width - 125.0, self.window.size.height - 24.0], 12, &mut self.glyph_cache, &c, gl);
                },
                GameStatus::Died => {
                    draw_text("DEAD", [self.window.size.width / 3.0, self.window.size.height / 2.0 - 32.0], 32, &mut self.glyph_cache, &c, gl);
                    let curr_score = format!("Score: {:?}", self.score);
                    draw_text(curr_score.as_str(), [self.window.size.width / 3.0, self.window.size.height / 2.0 + 18.0], 18, &mut self.glyph_cache, &c, gl);
                }, 
                GameStatus::Win => {
                    draw_text("Win", [self.window.size.width / 3.0, self.window.size.height / 2.0 - 32.0], 32, &mut self.glyph_cache, &c, gl);
                    let curr_score = format!("Score: {:?}", self.score);
                    draw_text(curr_score.as_str(), [self.window.size.width / 3.0, self.window.size.height / 2.0 + 18.0], 18, &mut self.glyph_cache, &c, gl);
                }
            }
        });

    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.uptime += 1;
        
        // First we update Players health
        for enemy in self.enemies.iter_mut() {
            enemy.update(args.dt, self.window.size);
            if enemy.defense_breached {
                self.player.health -= enemy.health;
                enemy.health = 0.0;
            }
        }
        
        // If players health becomes zero: lost.
        if self.player.health <= 0.0
        {
            self.game_status = GameStatus::Died;
        }
        
        // Update players parameters as other updattions use this parameters.
        // self.rotation += self.speed * args.dt;
        self.player.update(args.dt, self.window.size);

        // Update shoot cooldown and fire bullet if reloaded.
        if self.shoot_cooldown > 0 {
            self.shoot_cooldown -= 1;
        }

        if self.fire_bullet && self.ammo > 0 {
            self.fire_bullet = false;
            self.bullets.push(Bullet::new(self.player.pos.x, self.player.pos.y));
            self.ammo -= 1;
        }

        self.spawn_enemies();

        for bullet in self.bullets.iter_mut()
        {
            for enemy in self.enemies.iter_mut() {
                if enemy.health == 0.0 {
                    continue;
                }
                if true == bullet.collides(enemy) {
                    enemy.health = 0.0;
                    bullet.destroy = true;
                    self.score += 1;
                    self.ammo += 2;
                    break;
                }
            }
        }

        self.enemies.retain(|enemy| enemy.health > 0.0);

        self.bullets.retain(|bullet| false == bullet.destroy);
        for bullet in self.bullets.iter_mut() {
            bullet.update(args.dt, self.window.size);
        }
        
        
    }

    fn spawn_enemies (&mut self) {

        let mut difficulty = self.uptime / 50; // We increase difficulty after every interval.
        
        // Max difficulty is spawning enemies after every 100 updates.
        // Hence we cannot have difficulty less more than 400.
        if difficulty > 400 {
            difficulty = 400;
        }

        if self.uptime % (500 - difficulty) == 1
        {
            let mut rng = thread_rng(); 
            let arr: [f32; 10] = rng.gen(); // Every column has its own random number.
            for i in 0..9 {
                if arr[i] > 0.7 {
                    self.enemies.push(Enemy::new(self.enemy_spawn_columns[i], 0.0));
                }
            }
        }
    }

    pub fn input (&mut self, button: &Button, press_event: bool) {

        let mut direction: Direction = self.player.dir.clone();
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => direction = geom::Direction::NORTH,
                Key::Down => direction = geom::Direction::SOUTH,
                Key::Right => direction = geom::Direction::EAST,
                Key::Left => direction = geom::Direction::WEST,
                Key::Space => {
                    if self.shoot_cooldown <= 0 && press_event{
                        self.fire_bullet = true;
                    }
                    return;
                }, // Fire bullets!
                _ => (), // Ignore all other
            }
        }

        // print!("direction key: {:?} press_action: {:?} player direction: {:?}\n", direction, press_event, self.player.dir);

        if direction == self.player.dir && !press_event {
            self.player.stop_movement = true;
        }
        else if press_event {
            self.player.stop_movement = false;               
        }
        if press_event {
            self.player.stop_movement = false;
            self.player.movement(direction);
        }

    }
}
