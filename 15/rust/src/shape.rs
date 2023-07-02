use crate::grid::{Grid, Reading, Position, manhattan};

#[derive(Debug)]
pub struct Diamond {
    center: Position,
    radius: i32,
}

impl Diamond {
    pub fn new(center: Position, radius: i32) -> Diamond {
        Diamond { center, radius }
    }

    fn build_from_reading(r: Reading) -> Diamond {
        Diamond { center: r.sensor, radius: r.distance}
    }

    pub fn build_from_grid(g: Grid) -> Vec<Diamond> {
        let mut diamonds: Vec<Diamond> = Vec::new();
        for r in g.readings {
            diamonds.push(Diamond::build_from_reading(r));
        }
        diamonds
    }

    pub fn overlaps_with_square(&self, sq: &Square) -> bool {
        for corner in self.corners() {
            if sq.point_within(&corner) {
                return true
            }
        }
        false
    }

    fn point_within(&self, p: &Position) -> bool {
        // Returns true if the given point falls within the diamond.
        manhattan(p, &self.center) <= self.radius
    }

    fn corners(&self) -> Vec<Position> {
        // Returns the four corners of the diamond.
        let mut corners: Vec<Position> = Vec::new();
        corners.push(Position{x: self.center.x + self.radius, y: self.center.y});
        corners.push(Position{x: self.center.x - self.radius, y: self.center.y});
        corners.push(Position{x: self.center.x, y: self.center.y + self.radius});
        corners.push(Position{x: self.center.x, y: self.center.y - self.radius});
        corners
    }

    pub fn around(&self) -> Vec<Position> {
        // Returns points that aren't in the diamond but run along the edges.
        let mut points = Vec::new();
        // Start just outside the left edge.
        // We intentionally don't add this point yet; we'll end up adding it as we finish the loop.
        let mut current_point = Position{x: self.center.x - self.radius - 1, y: self.center.y};
        // Move up the top-left edge.
        while current_point.y < (self.center.y + self.radius + 1) {
            current_point.y += 1;
            current_point.x += 1;
            points.push(current_point.clone());
        }
        // Move down the top-right edge.
        while current_point.x < (self.center.x + self.radius + 1) {
            current_point.x += 1;
            current_point.y -= 1;
            points.push(current_point.clone());
        }
        // Move down the bottom-right edge.
        while current_point.y > (self.center.y - self.radius - 1) {
            current_point.y -= 1;
            current_point.x -= 1;
            points.push(current_point.clone());
        }
        // Move up the bottom-left edge.
        while current_point.x > (self.center.x - self.radius - 1) {
            current_point.x -= 1;
            current_point.y += 1;
            points.push(current_point.clone());
        }
        points
    }
}

pub struct Square {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Square {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Square {
        // Ensure that x2 is always greater than x1
        let (mut x1, mut x2) = (x1, x2);
        if x1 > x2 {
            (x1, x2) = (x2, x1)
        }
        // Ensure that y2 is always greater than y1
        let (mut y1, mut y2) = (y1, y2);
        if y1 > y2 {
            (y1, y2) = (y2, y1)
        }
        Square { x1, y1, x2, y2 }
    }

    fn point_within(&self, p: &Position) -> bool {
        // Returns true if the given point falls within the square.
        p.x >= self.x1 && p.x <= self.x2 && p.y >= self.y1 && p.y <= self.y2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_around() {
        let d = Diamond::new(Position{x: 0, y: 0}, 1);
        let around = d.around();
        println!("{:?}", around);
        assert_eq!(around.len(), 8);
        assert!(around.contains(&Position{x: 0, y: 2}));
        assert!(around.contains(&Position{x: 1, y: 1}));
        assert!(around.contains(&Position{x: 2, y: 0}));
        assert!(around.contains(&Position{x: 1, y: -1}));
        assert!(around.contains(&Position{x: 0, y: -2}));
        assert!(around.contains(&Position{x: -1, y: -1}));
        assert!(around.contains(&Position{x: -2, y: 0}));
        assert!(around.contains(&Position{x: -1, y: 1}));
    }
}