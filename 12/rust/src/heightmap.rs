#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point (char);

#[derive(Debug)]
pub struct HeightMap {
    points: Vec<Vec<Point>>,
    start_pos: Position,
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Path {
    points: Vec<Position>,
}

impl HeightMap {
    pub fn build_from_str(s: &str) -> Result<Self, String> {
        let mut points = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Point(c));
            }
            points.push(row);
        }
        let mut map = HeightMap { points , start_pos: Position{x: 0, y: 0}};
        let start_pos = map.find_start_pos()?;
        map.start_pos = start_pos;
        Ok(map)
    }

    fn find_start_pos(&self) -> Result<Position, String> {
        for (y, row) in self.points.iter().enumerate() {
            for (x, point) in row.iter().enumerate() {
                if point == &Point('S') {
                    return Ok(Position{x, y});
                }
            }
        }
        Err(String::from("No start position found"))
    }

    fn at(&self, pos: &Position) -> Option<&Point> {
        self.points.get(pos.y)?.get(pos.x)
    }

    fn valid_elevation_change(&self, a: &Position, b: &Position) -> Result<bool, String> {
        let a_height = match self.at(a) {
            Some(Point('S')) => 'a' as u8,
            Some(Point('E')) => 'z' as u8,
            Some(p) => p.0 as u8,
            None => return Err(String::from("a is out of bounds")),
        };
        let b_height = match self.at(b) {
            Some(Point('S')) => 'a' as u8,
            Some(Point('E')) => 'z' as u8,
            Some(p) => p.0 as u8,
            None => return Err(String::from("b is out of bounds")),
        };
        Ok((a_height + 1) >= b_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_and_at() {
        let contents = "Sab\ncEd\nghi";
        let map = HeightMap::build_from_str(contents).unwrap();
        assert_eq!(map.at(&Position{x: 0, y: 0}), Some(&Point('S')));
        assert_eq!(map.at(&Position{x: 2, y: 2}), Some(&Point('i')));
        assert_eq!(map.at(&Position{x: 2, y: 0}), Some(&Point('b')));
        assert_eq!(map.at(&Position{x: 0, y: 2}), Some(&Point('g')));
        assert_eq!(map.at(&Position{x: 3, y: 2}), None);
        assert_eq!(map.at(&Position{x: 0, y: 3}), None);
    }

    #[test]
    fn test_valid_elevation_change() {
        let contents = "Sab\ncEd\ngxy";
        let map = HeightMap::build_from_str(contents).unwrap();
        // We can go from the start position to the 'a', but not to the 'c'
        assert_eq!(map.valid_elevation_change(&Position{x: 0, y: 0}, &Position{x: 0, y: 1}), Ok(false));
        assert_eq!(map.valid_elevation_change(&Position{x: 0, y: 0}, &Position{x: 1, y: 0}), Ok(true));
        // We can go from the 'c' to the 'S' or 'a' or 'b', but not to the 'g'
        for p in vec![&Position{x: 0, y: 0}, &Position{x: 1, y: 0}, &Position{x: 2, y: 0}] {
            assert_eq!(map.valid_elevation_change(&Position{x: 0, y: 1}, p), Ok(true));
        }
        assert_eq!(map.valid_elevation_change(&Position{x: 0, y: 1}, &Position{x: 0, y: 2}), Ok(false));
        // We can go from the 'y' to any position
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(map.valid_elevation_change(&Position{x: 2, y: 2}, &Position{x: i, y: j}), Ok(true));
            }
        }
        // We can't get from 'x' to E ('z') though
        assert_eq!(map.valid_elevation_change(&Position{x: 1, y: 2}, &Position{x: 1, y: 1}), Ok(false));
    }
}