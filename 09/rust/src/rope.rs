use std::collections::hash_set::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

pub struct Rope {
    head: Position,
    tail: Position,
    tail_history: Vec<Position>,
}

impl Rope {
    pub fn new_at_origin() -> Rope {
        Rope {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
            tail_history: vec![Position { x: 0, y: 0 }],
        }
    }

    fn move_head(&mut self, direction: Direction, distance: i32) {
        match direction {
            Direction::Up => self.head.y += distance,
            Direction::Down => self.head.y -= distance,
            Direction::Left => self.head.x -= distance,
            Direction::Right => self.head.x += distance,
        }
    }

    fn update_tail(&mut self) -> bool {
        // Update the positin of the tail based on the position of the head. Return true if the tail was moved, false otherwise.

        // A vector describing the path from the tail to the head.
        let vector = Position {
            x: self.head.x - self.tail.x,
            y: self.head.y - self.tail.y,
        };
        match vector {
            // The head and tail are on the same space.
            Position { x: 0, y: 0 } => {
                self.tail_history.push(self.tail.clone());
                return false;
            },
            // The head is directly above or below the tail by just one space.
            Position { x: 0, y } if y.abs() <= 1 => {
                self.tail_history.push(self.tail.clone());
                return false;
            },
            // The head is directly to the left or right of the tail by just one space.
            Position { x, y: 0 } if x.abs() <= 1 => {
                self.tail_history.push(self.tail.clone());
                return false;
            },
            // The head is diagonal to the tail by just one space.
            Position { x, y } if x.abs() <= 1 && y.abs() <= 1 => {
                self.tail_history.push(self.tail.clone());
                return false;
            }

            // The head is directly above or below the tail by multiple spaces.
            Position { x: 0, y } => {
                if y > 0 {
                    self.tail.y += 1;
                } else {
                    self.tail.y -= 1;
                }
                self.tail_history.push(self.tail.clone());
                return true;
            }
            // The head is directly left or right of the tail by multiple spaces.
            Position { x, y: 0 } => {
                if x > 0 {
                    self.tail.x += 1;
                } else {
                    self.tail.x -= 1;
                }
                self.tail_history.push(self.tail.clone());
                return true;
            }
            // The head is diagonal to the tail by multiple spaces.
            Position { x, y } => {
                if x > 0 {
                    self.tail.x += 1;
                } else {
                    self.tail.x -= 1;
                }
                if y > 0 {
                    self.tail.y += 1;
                } else {
                    self.tail.y -= 1;
                }
                self.tail_history.push(self.tail.clone());
                return true;
            },
        };
    }

    pub fn do_instruction(&mut self, instruction: Instruction) {
        self.move_head(instruction.direction, instruction.distance);
        // Update the tail repeatedly until it no longer needs to move.
        while self.update_tail() {}
    }

    pub fn get_tail_history(&self) -> Vec<Position> {
        self.tail_history.clone()
    }

    pub fn get_count_uniq_tail_spaces(&self) -> usize {
        let mut uniq_spaces = HashSet::new();
        for space in self.tail_history.iter() {
            uniq_spaces.insert(space.clone());
        }
        uniq_spaces.len()
    }
}

pub struct Instruction {
    direction: Direction,
    distance: i32,
}

impl Instruction {
    fn build_from_str(s: &str) -> Result<Instruction, String> {
        let s = s.trim();
        assert!(s.len() >= 3);
        let (direction, distance) = s.split_at(1);
        let direction = direction.trim();
        let distance = distance.trim();
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(String::from("Invalid direction")),
        };
        let distance = match distance.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(String::from("Invalid distance")),
        };
        Ok(Instruction {
            direction,
            distance,
        })
    }

    pub fn build_from_file_contents(contents: String) -> Result<Vec<Instruction>, String> {
        let instructions = contents
            .lines()
            .into_iter()
            .map(|l| Instruction::build_from_str(l))
            .collect::<Result<Vec<Instruction>, String>>()?;
        Ok(instructions)
    }
}
