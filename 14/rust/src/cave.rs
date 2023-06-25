use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    fn positions_between(&self, other: &Position) -> Result<HashSet<Position>, String> {
        if self.x != other.x && self.y != other.y {
            return Err(format!("Invalid endpoints: {:?}, {:?}", self, other));
        }
        let mut positions = HashSet::new();
        if self.x == other.x {
            let min_y = std::cmp::min(self.y, other.y);
            let max_y = std::cmp::max(self.y, other.y);
            for y in min_y..=max_y {
                positions.insert(Position { x: self.x, y });
            }
            return Ok(positions);
        } else { // self.y == other.y
            let min_x = std::cmp::min(self.x, other.x);
            let max_x = std::cmp::max(self.x, other.x);
            for x in min_x..=max_x {
                positions.insert(Position { x, y: self.y });
            }
            return Ok(positions);
        }
    }
}

#[derive(Debug)]
pub struct Cave {
    pub blockers: HashSet<Position>,
}

impl Cave {
    pub fn build_from_text(text: &str) -> Result<Self, String> {
        let lines = text.lines();
        let mut blockers = HashSet::new();
        for line in lines {
            let endpoints = Self::parse_line(line)?;
            for i in 0..endpoints.len() - 1 {
                let new_blockers = endpoints[i].positions_between(&endpoints[i + 1])?;
                blockers.extend(new_blockers);
            }
        }
        Ok(Cave{ blockers })
    }

    fn parse_line(line: &str) -> Result<Vec<Position>, String> {
        let position_strs: Vec<Position> = line
            .split(" -> ")
            .into_iter()
            .map(|s| s.trim())
            .map(|s| {
                let parts: Vec<i32> = s.split(',')
                    .map(|s| {
                        s
                            .parse::<i32>()
                            .map_err(|e| e.to_string())
                    })
                    .collect::<Result<Vec<i32>, String>>()?;
                if parts.len() != 2 {
                    return Err(format!("Invalid position: {}", s));
                }
                Ok(Position { x: parts[0], y: parts[1] })
            })
            .collect::<Result<Vec<Position>, String>>()?;
        Ok(position_strs)
    }

    pub fn lowest_y(&self) -> Option<i32> {
        let mut lowest: Option<i32> = None;
        for blocker in &self.blockers {
            if let Some(lowest_y) = lowest {
                if blocker.y > lowest_y {
                    lowest = Some(blocker.y);
                }
            } else {
                lowest = Some(blocker.y);
            }
        }
        lowest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_from_text_single_line() {
        let cave = Cave::build_from_text("498,4 -> 498,6 -> 496,6").unwrap();
        assert_eq!(cave.blockers.len(), 5);
        for position in vec![
            Position { x: 498, y: 4 },
            Position { x: 498, y: 5 },
            Position { x: 498, y: 6 },
            Position { x: 497, y: 6 },
            Position { x: 496, y: 6 },
        ] {
            assert!(cave.blockers.contains(&position));
        }
    }

    #[test]
    fn test_build_from_text_multiple_lines() {
        let text = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        let cave = Cave::build_from_text(text).unwrap();
        assert_eq!(cave.blockers.len(), 20);
        for position in vec![
            // Line 1
            Position { x: 498, y: 4 },
            Position { x: 498, y: 5 },
            Position { x: 498, y: 6 },
            Position { x: 497, y: 6 },
            Position { x: 496, y: 6 },
            // Line 2
            Position { x: 503, y: 4 },
            Position { x: 502, y: 4 },
            Position { x: 502, y: 5 },
            Position { x: 502, y: 6 },
            Position { x: 502, y: 7 },
            Position { x: 502, y: 8 },
            Position { x: 502, y: 9 },
            Position { x: 501, y: 9 },
            Position { x: 500, y: 9 },
            Position { x: 499, y: 9 },
            Position { x: 498, y: 9 },
            Position { x: 497, y: 9 },
            Position { x: 496, y: 9 },
            Position { x: 495, y: 9 },
            Position { x: 494, y: 9 },
        ] {
            assert!(cave.blockers.contains(&position));
        }
    }
}