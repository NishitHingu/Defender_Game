use graphics::{ellipse, Transformed, color};

use crate::geom::Position;

use super::GameObject;

const BULLET_SIZE: f64 = 2.0;
const BULLET_SPEED: f64 = 2.0;

pub struct Bullet {
    pos: Position,
    pub destroy: bool,
    size: f64,
}

impl Bullet {
    pub fn new (x: f64, y: f64) -> Bullet {
        Bullet { 
            pos: Position::new(x,y), 
            destroy: false,
            size: BULLET_SIZE
        }
    }
}

impl GameObject for Bullet {
    fn position(&self) -> &Position { &self.pos }
    fn radius(&self) -> f64 { self.size }

    fn render(&self, ctxt: &graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        let transform = ctxt.transform.trans(self.pos.x, self.pos.y);
        let radius = self.radius();
        ellipse(color::RED, [0.0, 0.0, radius, radius], transform, gl);
    }

    fn update(&mut self, _dt: f64, size: piston::Size) {
        self.pos.y -= BULLET_SPEED;

        if self.pos.y <= 0.0 || self.pos.y > size.height {
            self.destroy = true;
        }
    }
}