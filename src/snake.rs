use crate::direction::Direction;
use crate::point::Point;

// Represents the snake, or the player in the game
// Has a body, a Vec of Points that the snake occupies, direction it is moving,
// and if it is currently growing (from eating)
#[derive(Debug)]
pub struct Snake {
    body: Vec<Point>,
    direction: Direction,
    growing: bool,
}

impl Snake {
    pub fn new(start: Point, length: u16, direction: Direction) -> Self {
        let opposite = direction.opposite();
        let body: Vec<Point> = (0..length)
            .into_iter()
            .map(|i| start.transform(opposite, i))
            .collect();

        Self {body, direction, growing: false }
    }

    // Returns the head point of the snake
    pub fn get_head_point(&self) -> Point {
        self.body.first().unwrap().clone()
    }

    // Returns a copy of the snake's body (Vec of points)
    pub fn get_body_points(&self) -> Vec<Point> {
        self.body.clone()
        // Vec doesn't implement copy, so we use clone instead
        // Without clone, it would try to move the value, which isn't allowed (ownership!)
        // Could return an immutable reference (&Vec) instead, which would be less mem intensive
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
        // Clone method again!
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        // Returns true if the point is in body
        self.body.contains(point)
    }

    // Only argument is a mutable self reference, this is a void function!
    pub fn slither(&mut self) {
        // Add a Point to body, the new head head position as the snake moves
        self.body.insert(0, self.body.first().unwrap().transform(self.direction, 1));

        // If the snake is NOT growing on this turn, remove the final tail space from body
        // Else, set growing field to false, as snake is no longer growing
        if !self.growing {
            self.body.remove(self.body.len() - 1);
        } else {
            self.growing = false;
        }
    }

    // Sets the snake's direction
    pub fn set_direction(&mut self, d: Direction) {
        self.direction = d;
    }

    // Sets the growing field to true
    pub fn grow(&mut self) {
        self.growing = true;
    }
}