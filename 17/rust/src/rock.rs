use std::fmt::Display;
use crate::point::Point;

const DEFAULT_ROCK_TEXT: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";


#[derive(Debug, Clone)]
pub struct Rock {
    pub points: Vec<Point>,
}

impl Rock {
    pub fn build_from_text(text: &str) -> Rock {
        let mut points = Vec::new();
        for (line_number, line) in text.lines().rev().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            for (char_number, c) in chars.iter().enumerate() {
                if *c == '#' {
                    points.push(Point { x: char_number as i32, y: line_number as i32 });
                }
            }
        }
        Rock{ points }
    }

    pub fn build_multiple_from_text(text: &str) -> Vec<Rock> {
        let text_blocks = text.trim().split("\n\n");
        text_blocks.map(|block| Rock::build_from_text(block)).collect()
    }

    pub fn add_x(&mut self, x: i32) {
        for point in self.points.iter_mut() {
            point.x += x;
        }
    }

    pub fn add_y(&mut self, y: i32) {
        for point in self.points.iter_mut() {
            point.y += y;
        }
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = self.points.iter().map(|p| p.x).max().unwrap();
        let max_y = self.points.iter().map(|p| p.y).max().unwrap();
        for y in 0..=max_y {
            // Adjust for the fact that we're printing from the top, but the points are relative to the bottom.
            let y = max_y - y;
            for x in 0..=max_x {
                if self.points.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

/// Loops over a set of rocks and dispenses them in order, cycling back to the beginning as needed.
#[derive(Clone)]
pub struct RockDropper {
    rocks: Vec<Rock>,
    drop_count: usize,
}

impl RockDropper {
    pub fn new_with_default() -> RockDropper {
        let rocks = Rock::build_multiple_from_text(DEFAULT_ROCK_TEXT);
        RockDropper { rocks, drop_count: 0 }
    }

    pub fn drop(&mut self) -> Rock {
        let index = self.drop_count % self.rocks.len();
        self.drop_count += 1;
        self.rocks[index].clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_from_text() {
        let text = "###";
        let rock = Rock::build_from_text(text);
        let expected_points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
        ];
        assert_eq!(rock.points, expected_points);

        let text = ".#.\n###\n.#.";
        let rock = Rock::build_from_text(text);
        let expected_points = vec![
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ];
        assert_eq!(rock.points, expected_points);

        let text = "#\n#\n#\n#";
        let rock = Rock::build_from_text(text);
        let expected_points = vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ];
        assert_eq!(rock.points, expected_points);
    }

    #[test]
    fn test_build_multiple_from_text() {
        let text = DEFAULT_ROCK_TEXT;
        let rocks = Rock::build_multiple_from_text(text);
        assert_eq!(rocks.len(), 5);
        assert_eq!(rocks[0].points.len(), 4);
    }
}