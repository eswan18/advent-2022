use std::collections::HashSet;
use crate::rock::{Rock, RockDropper};
use crate::point::Point;

pub struct Chamber {
    set_rocks: Vec<Rock>,
    rock_dropper: RockDropper,
    jet_pattern: Vec<char>,
    jet_count: usize,
}

impl Chamber {
    pub fn new(jet_pattern: String) -> Chamber {
        let set_rocks = vec![];
        let active_rock = None;
        let jet_pattern = jet_pattern.chars().collect();
        let rock_dropper = RockDropper::new_with_default();
        Chamber { set_rocks, rock_dropper, active_rock, jet_pattern, jet_count: 0 }
    }

    fn all_points(&self) -> HashSet<Point> {
        self.set_rocks
            .iter()
            .flat_map(|r| r.points.iter().map(|p| p.clone()))
            .collect()
    }

    pub fn drop_rock(&mut self) {
        let mut rock = self.rock_dropper.drop();
        self.align_dropped_rock(&mut rock);
        loop {
            // Let the rock be pushed by a jet.
            self.push_jet(&mut rock);
            // Let the rock fall one square.
            let stuck = self.push_down(&mut rock).is_err();
            // If it can't fall any more, we're done and can move on.
            if stuck {
                break;
            }
        }
        self.set_rocks.push(rock);
    }

    /// Push the rock one square in the direction of the jet if it isn't obstructed.
    fn push_jet(&mut self, rock: &mut Rock) -> Result<(), String> {
        let index = self.jet_count % self.jet_pattern.len();
        let jet = self.jet_pattern[index];
        self.jet_count += 1;
        match jet {
            '<' => {
                if self.is_obstructed_left(&rock) {
                    return Err(format!("Rock {:?} is obstructed to the left", rock))
                }
                rock.add_x(-1);
            }
            '>' => {
                if self.is_obstructed_right(&rock) {
                    return Err(format!("Rock {:?} is obstructed to the right", rock))
                }
                rock.add_x(1);
            }
            _ => panic!("Unexpected jet direction: {}", jet),
        }
        Ok(())
    }

    /// Push the rock down one square if it isn't obstructed.
    fn push_down(&self, rock: &mut Rock) -> Result<(), String> {
        if self.is_obstructed_below(&rock) {
            return Err(format!("Rock {:?} is obstructed below", rock))
        }
        rock.add_y(-1);
        Ok(())
    }

    /// Takes a newly-dropped rock and puts it in the right place in the chamber: 2 units from the left edge,
    /// and 3 units above the highest rock or the floor.
    fn align_dropped_rock(&self, rock: &mut Rock) {
        let y_coord = self.highest_point() + 3;
        let x_coord = 2;
        rock.add_x(x_coord);
        rock.add_y(y_coord);
    }

    /// Get y-coordinate of the highest object (or the floor).
    pub fn highest_point(&self) -> i32 {
        let all_points = self.all_points();
        all_points.iter().map(|p| p.y).max().unwrap_or(0)
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all_points = self.all_points();
        let max_x = all_points.iter().map(|p| p.x).max().unwrap();
        let max_y = all_points.iter().map(|p| p.y).max().unwrap();
        for y in 0..=max_y {
            // Adjust for the fact that we're printing from the top, but the points are relative to the bottom.
            let y = max_y - y;
            for x in 0..=max_x {
                if all_points.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "----- FLOOR -----")?;
        Ok(())
    }
}