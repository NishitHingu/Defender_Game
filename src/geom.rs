#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    WEST,
    NORTH,
    EAST,
    SOUTH
}

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position { x, y }
    }
}

pub fn restrict_to_bounds(pos: &mut Position, bounds: [f64; 2]) {
    // Make sure movement is within the window bounds.
    
    if pos.x < 0.0 {
        pos.x = bounds[0];
    } else if bounds[0] - pos.x <= 0.0 {
        pos.x = 0.0;
    }

    if pos.y < 0.0 {
        pos.y = 0.0;
    } else if bounds[1] - pos.y <= 0.0 {
        pos.y = bounds[1];
    }

}