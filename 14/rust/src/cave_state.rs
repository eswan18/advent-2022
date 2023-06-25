use std::collections::HashSet;
use crate::cave::{Cave, Position}; 

#[derive(Debug)]
pub struct CaveState {
    cave: Cave,
    sand_at_rest: HashSet<Position>,
    current_sand: Option<Position>,
    abyss_begins: i32,
    pub in_abyss: i32,
    pub n_dropped: i32,
}

impl CaveState {
    pub fn build_from_text(text: &str) -> Result<Self, String> {
        let cave = Cave::build_from_text(text)?;
        let sand_at_rest = HashSet::new();
        let current_sand = None;
        let in_abyss = 0;
        let abyss_begins = cave.lowest_y().ok_or("No blockers in cave")?;
        let n_dropped = 0;
        Ok(CaveState { cave, sand_at_rest, current_sand, abyss_begins, in_abyss, n_dropped })
    }

    pub fn drop_sand(&mut self) {
        self.current_sand = Some(Position { x: 500, y: 0 });
        self.n_dropped += 1;
        while self.update() {}
    }

    pub fn drop_sand_until_in_abyss(&mut self) {
        while self.in_abyss == 0 {
            self.drop_sand();
        }
    }

    pub fn drop_sand_until_blocked(&mut self) {
        while !self.sand_at_rest.contains(&Position{x: 500, y: 0}) {
            self.drop_sand();
        }
    }

    // Move one piece of sand down one step. Return false if there is no sand to move.
    fn update(&mut self) -> bool {
        if let Some(current_pos) = self.current_sand.take() {
            match current_pos {
                // Check if we've fallen past the lowest blocker, in which case we're in the abyss.
                // By waiting until the sand is 3 squares into the abyss, we let the sand settle on the cave "bottom" if we've added one.
                Position{ x: _, y } if y > self.abyss_begins + 3 => {
                    self.in_abyss += 1;
                    return false;
                },
                // Try to fall straight down.
                Position{ x, y } if !self.is_blocked(Position{ x, y: y + 1 }) => {
                    self.current_sand = Some(Position{ x, y: y + 1 });
                    return true;
                },
                // Try to fall down and to the left.
                Position{ x, y } if !self.is_blocked(Position{ x: x - 1, y: y + 1 }) => {
                    self.current_sand = Some(Position{ x: x - 1, y: y + 1 });
                    return true;
                },
                // Try to fall down and to the right.
                Position{ x, y } if !self.is_blocked(Position{ x: x + 1, y: y + 1 }) => {
                    self.current_sand = Some(Position{ x: x + 1, y: y + 1 });
                    return true;
                },
                // If all these are blocked, the sand is at rest.
                Position{ x, y } => {
                    self.sand_at_rest.insert(Position{ x, y });
                    return false;
                },

            }
        } else {
            // There is no sand in motion, so we just return false.
            return false
        };
    }

    fn is_blocked(&self, p: Position) -> bool {
        self.cave.blockers.contains(&p) || self.sand_at_rest.contains(&p)
    }

    pub fn add_cave_bottom(&mut self) {
        // Add an additional line of blockers at the bottom of the cave.
        let lowest_y = self.cave.lowest_y().unwrap();
        let min_x = self.cave.blockers.iter().map(|p| p.x).min().unwrap();
        let max_x = self.cave.blockers.iter().map(|p| p.x).max().unwrap();
        // Add blockers all along the bottom, with enough extra margin on each side to
        // keep things from ever falling off the sides.
        for x in (min_x - lowest_y - 1)..(max_x + lowest_y + 1) {
            self.cave.blockers.insert(Position { x, y: lowest_y + 2 });
        }
    }
}