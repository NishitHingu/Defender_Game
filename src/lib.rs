use geom::Direction;
use graphics::color::BLACK;
use models::GameObject;
use models::bullet::Bullet;
use piston::input::{RenderArgs, UpdateArgs, Button, Key};
use models::player::Player;
use models::enemy::Enemy;

mod geom;
pub mod config;
pub mod models;

const SHOOT_COOLDOWN: u32 = 50;
pub struct App {
    pub window: config::GraphicsConfig, // OpenGL drawing backend.
    player: Player,
    enemies: Vec<Enemy>,
    bullets: Vec<Bullet>,
    fire_bullet: bool,
    score: u32,
    shoot_cooldown: u32,
    rotation: f64,  // Rotation for the square.
    speed: f64,
    uptime: u64,
}

impl App {
    pub fn new(window: config::GraphicsConfig) -> App {
        let size = window.size;

        let (x, y) = (f64::from(size.width / 2.0),
                      f64::from(size.height / 2.0));

        let player = Player::new(x, y);

        App {
            window,
            player,
            enemies: Vec::new(),
            bullets: Vec::new(),
            fire_bullet: false,
            shoot_cooldown: SHOOT_COOLDOWN,
            score: 0,
            uptime: 0,
            speed: 0.0,
            rotation: 0.0,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.window.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            self.player.render(&c, gl);
            for enemy in self.enemies.iter() {
                enemy.render(&c, gl);
            }
        });

    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.uptime += 1;
        self.rotation += self.speed * args.dt;
        self.player.update(args.dt, self.window.size);



        if self.uptime % 500 == 1
        {
            self.enemies.push(Enemy::new(self.window.size.width / 4.0, 0.0));
            self.enemies.push(Enemy::new(self.window.size.width - (self.window.size.width / 4.0), 0.0));
        }

        self.enemies.retain(|enemy| enemy.position().y < self.window.size.height);
        for enemy in self.enemies.iter_mut() {
            enemy.update(args.dt, self.window.size);
        }
        
    }

    pub fn input (&mut self, button: &Button, press_event: bool) {
        self.player.stop_movement = !press_event;

        let mut direction: Direction = self.player.dir;
        if let Button::Keyboard(key) = *button {
            match key {
                Key::Up => direction = geom::Direction::NORTH,
                Key::Down => direction = geom::Direction::SOUTH,
                Key::Right => direction = geom::Direction::EAST,
                Key::Left => direction = geom::Direction::WEST,
                Key::Space => {
                    if self.shoot_cooldown <= 0 {
                        self.fire_bullet = true;
                    }
                }, // Fire bullets!
                _ => (), // Ignore all other
            }
        }

        if direction == self.player.dir {
            if !press_event {
                self.player.stop_movement = true;
            }
            else {
                self.player.stop_movement = false;   
            }
        }
        self.player.movement(direction);

    }
}
