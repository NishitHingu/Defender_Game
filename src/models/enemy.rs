use graphics::{Context, rectangle, Transformed, color::WHITE};
use opengl_graphics::GlGraphics;
use crate::geom::{self, Position};

use super::GameObject;
const ENEMY_SIZE: f64 = 20.0;
const ENEMY_SPEED: f64 = 1.0;
const ENEMY_HEALTH: f64 = 100.0;

pub struct Enemy {
    pos: Position,
    pub defense_breached: bool,
    pub health: f64,
    size: f64,
}

impl Enemy {
    pub fn new (x: f64, y: f64) -> Enemy {
        Enemy {
            pos: geom::Position::new(x,y),
            defense_breached: false,
            health: ENEMY_HEALTH,
            size: ENEMY_SIZE,
        }
    }
}

impl GameObject for Enemy {
    fn position(&self) -> &geom::Position { &self.pos }
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let shape = rectangle::square(0.0, 0.0, self.size);

        // Rotate the player to the direction they're facing
        let dir = 90.0;

        let radius = self.radius();
        let transform = ctxt.transform
            .trans(self.pos.x, self.pos.y)
            .rot_deg(dir)
            .trans(-radius, -radius);

        rectangle(WHITE, shape, transform, gl);
    }

    fn update(&mut self, dt: f64, size: piston::Size) {
        // print!("{:?} {:?} \n", (self.dir), self.pos);
        self.pos.y += ENEMY_SPEED;

        if self.pos.y < 0.0 || self.pos.y >= size.height
        {
            self.defense_breached = true;
        }
       
    }
}