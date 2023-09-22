use crate::direction::Direction;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]

// Represents a point on a 2-D grid, the playing space
// Restrictions: both fields must be positive ints
pub struct Point {
    pub x: u16,
    pub y: u16,
}

// Methods for the Point struct
// Called the implementation of the struct (hence "impl")
impl Point {
    // Creates a new point with the given coordinates
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    // Public method to transform the point by times spaces in direction
    pub fn transform(&self, direction: Direction, times: u16) -> Self {
        let times = times as i16;
        let transformation = match direction {
            Direction::Up => (0, -times),
            Direction::Right => (times, 0),
            Direction::Down => (0, times),
            Direction::Left => (-times, 0),
        };

        Self::new(
            Self::transform_value(self.x, transformation.0),
            Self::transform_value(self.y, transformation.1),
        )
    }

    // Private method (methods are private by default)
    fn transform_value(value: u16, by: i16) -> u16 {
        if by < 0 && by.abs() as u16 > value {
            panic!("Transforming value {} by {} would result in a negative number!", value, by);
        } else {
            (value as i16 + by) as u16
        }
    }
}