use graphics::{Context, polygon, Transformed, color};
use opengl_graphics::GlGraphics;
use crate::geom::{self, restrict_to_bounds};
use crate::geom::Direction;

use super::GameObject;

const PLAYER_SPEED: f64 = 1.75;
const PLAYER_SIZE: f64 = 25.0;
const PLAYER_HEALTH: f64 = 1000.0;

pub struct Player {
    pub pos: geom::Position,
    pub dir: geom::Direction,
    pub stop_movement: bool,
    pub health: f64,
    pub size: f64,
}

impl Player {
    pub fn new (x: f64, y: f64) -> Player {
        Player {
            dir: Direction::EAST,
            pos: geom::Position::new(x, y),
            stop_movement: true,
            health: PLAYER_HEALTH,
            size: PLAYER_SIZE,
        }
    }

    pub fn movement (&mut self, dir: Direction) {
        self.dir = dir;
    }

    pub fn reset (&mut self, x: f64, y:f64)
    {
        self.dir = Direction::EAST;
        self.pos.x = x;
        self.pos.y = y;
        self.stop_movement = true;
        self.health = PLAYER_HEALTH;
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

    fn update(&mut self, _dt: f64, size: piston::Size) {
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

        restrict_to_bounds(&mut self.pos, [size.width, size.height]);
    }
}

