use graphics::{Context, rectangle, polygon, Transformed, color};
use opengl_graphics::GlGraphics;
use crate::geom::{self, Position, restrict_to_bounds};
use crate::geom::Direction;

use super::GameObject;

const PLAYER_SPEED: f64 = 2.0;
const PLAYER_SIZE: f64 = 20.0;
// Drift for this long after movement key is released.
// You don't came to a hard stop in space!
const PLAYER_DRIFT: f64 = 0.2;

pub struct Player {
    pub pos: geom::Position,
    pub dir: geom::Direction,
    pub size: f64,
    pub dift_ttl: f64,
    pub stop_movement: bool,
}

impl Player {
    pub fn new (x: f64, y: f64) -> Player {
        Player {
            dir: Direction::EAST,
            dift_ttl: 0.0,
            stop_movement: true,
            pos: geom::Position::new(x, y),
            size: PLAYER_SIZE,
        }
    }

    pub fn movement (&mut self, dir: Direction) {
        self.dir = dir;
    }
}

impl GameObject for Player {
    fn position(&self) -> &geom::Position { &self.pos }
    fn radius(&self) -> f64 { self.size / 2.0 }

    fn render(&self, ctxt: &Context, gl: &mut GlGraphics) {
        // Render the player as a little square
        let shape = polygon::Polygon::new(color::RED);

        // Rotate the player to the direction they're facing
        let dir = 90.0;

        let radius = self.radius();
        let transform = ctxt.transform
            .trans(self.pos.x, self.pos.y)
            .rot_deg(dir)
            .trans(-radius, -radius);

        let points = [
            [0.0, radius],
            [self.size, self.size],
            [self.size, 0.0]
        ];

        shape.draw(
            &points,
            &ctxt.draw_state,
            transform,
            gl
        );
    }

    fn update(&mut self, dt: f64, size: piston::Size) {
        if self.stop_movement {
            return;
        }
        // print!("{:?} {:?} \n", (self.dir), self.pos);

        match self.dir {
            Direction::NORTH => self.pos.y -= PLAYER_SPEED,
            Direction::SOUTH => self.pos.y += PLAYER_SPEED,
            Direction::EAST => self.pos.x += PLAYER_SPEED,
            Direction::WEST => self.pos.x -= PLAYER_SPEED,
        }

        restrict_to_bounds(&mut self.pos, [size.width, size.height, size.width, size.height]);
    }
}

