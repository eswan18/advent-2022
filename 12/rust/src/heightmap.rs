use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point(char);

#[derive(Debug)]
pub struct HeightMap {
    points: Vec<Vec<Point>>,
    start_pos: Position,
    end_pos: Position,
    all_start_positions: Vec<Position>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
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
        let mut map = HeightMap {
            points,
            start_pos: Position { x: 0, y: 0 },
            end_pos: Position { x: 0, y: 0 },
            all_start_positions: vec![],
        };
        let start_pos = map.find_start_pos()?;
        map.start_pos = start_pos;
        let end_pos = map.find_end_pos()?;
        map.end_pos = end_pos;
        let all_start_positions = map.find_all_start_pos();
        map.all_start_positions = all_start_positions;
        Ok(map)
    }

    fn find_start_pos(&self) -> Result<Position, String> {
        for (y, row) in self.points.iter().enumerate() {
            for (x, point) in row.iter().enumerate() {
                if point == &Point('S') {
                    return Ok(Position { x, y });
                }
            }
        }
        Err(String::from("No start position found"))
    }

    fn find_end_pos(&self) -> Result<Position, String> {
        for (y, row) in self.points.iter().enumerate() {
            for (x, point) in row.iter().enumerate() {
                if point == &Point('E') {
                    return Ok(Position { x, y });
                }
            }
        }
        Err(String::from("No end position found"))
    }

    fn find_all_start_pos(&self) -> Vec<Position> {
        let mut positions = Vec::new();
        for (y, row) in self.points.iter().enumerate() {
            for (x, point) in row.iter().enumerate() {
                match point {
                    Point('S') => positions.push(Position { x, y }),
                    Point('a') => positions.push(Position { x, y }),
                    _ => (),
                }
            }
        }
        positions
    }

    fn at(&self, pos: &Position) -> Option<&Point> {
        self.points.get(pos.y)?.get(pos.x)
    }

    fn adjacent_positions(&self, pos: &Position) -> Vec<Position> {
        let mut positions = Vec::new();
        if pos.x > 0 {
            positions.push(Position {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        if pos.y > 0 {
            positions.push(Position {
                x: pos.x,
                y: pos.y - 1,
            });
        }
        if pos.x < self.points[0].len() - 1 {
            positions.push(Position {
                x: pos.x + 1,
                y: pos.y,
            });
        }
        if pos.y < self.points.len() - 1 {
            positions.push(Position {
                x: pos.x,
                y: pos.y + 1,
            });
        }
        positions
    }

    fn viable_moves_from(&self, pos: &Position) -> Vec<Position> {
        let moves: Vec<_> = self
            .adjacent_positions(pos)
            .into_iter()
            .filter(|p| self.valid_elevation_change(pos, p).unwrap_or(false))
            .collect();
        moves
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

    pub fn find_shortest_path(&self) -> Result<usize, String> {
        let starting_pos = self.start_pos.clone();
        let mut seen_positions = HashSet::new();
        seen_positions.insert(starting_pos);
        self.dfs(seen_positions)
    }

    pub fn find_shortest_path_including_all_start_positions(&self) -> Result<usize, String> {
        let mut seen_positions = HashSet::new();
        // Add all the values in self.all_starting_positions to seen_positions
        for pos in &self.all_start_positions {
            seen_positions.insert(pos.clone());
        }
        self.dfs(seen_positions)
    }

    // Depth-first search, starting at S and going to E
    fn dfs(&self, seen_positions: HashSet<Position>) -> Result<usize, String> {
        if seen_positions.contains(&self.end_pos) {
            return Ok(0);
        }

        let next_positions: HashSet<Position> = seen_positions
            .iter()
            .map(|p| self.viable_moves_from(p))
            .flatten()
            .filter(|p| !seen_positions.contains(p))
            .collect();
        self.dfs(next_positions).map(|n| n + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_and_at() {
        let contents = "Sab\ncEd\nghi";
        let map = HeightMap::build_from_str(contents).unwrap();
        assert_eq!(map.at(&Position { x: 0, y: 0 }), Some(&Point('S')));
        assert_eq!(map.at(&Position { x: 2, y: 2 }), Some(&Point('i')));
        assert_eq!(map.at(&Position { x: 2, y: 0 }), Some(&Point('b')));
        assert_eq!(map.at(&Position { x: 0, y: 2 }), Some(&Point('g')));
        assert_eq!(map.at(&Position { x: 3, y: 2 }), None);
        assert_eq!(map.at(&Position { x: 0, y: 3 }), None);
        assert_eq!(map.start_pos, Position { x: 0, y: 0 });
        assert_eq!(map.end_pos, Position { x: 1, y: 1 });
        assert_eq!(map.all_start_positions.len(), 2);
        assert!(map.all_start_positions.contains(&Position { x: 0, y: 0 }));
        assert!(map.all_start_positions.contains(&Position { x: 1, y: 0 }));
    }

    #[test]
    fn test_valid_elevation_change() {
        let contents = "Sab\ncEd\ngxy";
        let map = HeightMap::build_from_str(contents).unwrap();
        // We can go from the start position to the 'a', but not to the 'c'
        assert_eq!(
            map.valid_elevation_change(&Position { x: 0, y: 0 }, &Position { x: 0, y: 1 }),
            Ok(false)
        );
        assert_eq!(
            map.valid_elevation_change(&Position { x: 0, y: 0 }, &Position { x: 1, y: 0 }),
            Ok(true)
        );
        // We can go from the 'c' to the 'S' or 'a' or 'b', but not to the 'g'
        for p in vec![
            &Position { x: 0, y: 0 },
            &Position { x: 1, y: 0 },
            &Position { x: 2, y: 0 },
        ] {
            assert_eq!(
                map.valid_elevation_change(&Position { x: 0, y: 1 }, p),
                Ok(true)
            );
        }
        assert_eq!(
            map.valid_elevation_change(&Position { x: 0, y: 1 }, &Position { x: 0, y: 2 }),
            Ok(false)
        );
        // We can go from the 'y' to any position
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(
                    map.valid_elevation_change(&Position { x: 2, y: 2 }, &Position { x: i, y: j }),
                    Ok(true)
                );
            }
        }
        // We can't get from 'x' to E ('z') though
        assert_eq!(
            map.valid_elevation_change(&Position { x: 1, y: 2 }, &Position { x: 1, y: 1 }),
            Ok(false)
        );
    }

    #[test]
    fn test_viable_moves() {
        let contents = "Sab\ncEd\ngxy";
        let map = HeightMap::build_from_str(contents).unwrap();

        let viable_moves = map.viable_moves_from(&Position { x: 0, y: 0 });
        assert_eq!(viable_moves.len(), 1);
        assert_eq!(viable_moves[0], Position { x: 1, y: 0 });

        let viable_moves = map.viable_moves_from(&Position { x: 2, y: 0 });
        assert_eq!(viable_moves.len(), 1);
        assert_eq!(viable_moves[0], Position { x: 1, y: 0 });

        let viable_moves = map.viable_moves_from(&Position { x: 1, y: 2 });
        assert_eq!(viable_moves.len(), 2);
        assert!(viable_moves.contains(&Position { x: 0, y: 2 }));
        assert!(viable_moves.contains(&Position { x: 2, y: 2 }));

        let viable_moves = map.viable_moves_from(&Position { x: 0, y: 1 });
        assert_eq!(viable_moves.len(), 1);
        assert_eq!(viable_moves[0], Position { x: 0, y: 0 });
    }
}
