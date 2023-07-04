use std::collections::HashSet;
use crate::rock::{Rock, DEFAULT_ROCK_TEXT, RockDropper};
use crate::point::Point;

pub struct Chamber {
    rocks: Vec<Rock>,
    rock_dropper: RockDropper,
}

impl Chamber {
    pub fn new() -> Chamber {
        let rocks = vec![];
        let rock_dropper = RockDropper::new_with_default();
        Chamber { rocks, rock_dropper }
    }

    fn all_points(&self) -> HashSet<Point> {
        self.rocks
            .iter()
            .flat_map(|r| r.points.iter().map(|p| p.clone()))
            .collect()
    }

    pub fn drop_rock(&mut self) {
        let mut rock = self.rock_dropper.drop();
        self.align_new_rock(&mut rock);
        self.rocks.push(rock);
    }

    /// Takes a newly-dropped rock and puts it in the right place in the chamber: 2 units from the left edge,
    /// and 3 units above the highest rock or the floor.
    fn align_new_rock(&self, rock: &mut Rock) {
        let y_coord = self.highest_rock_or_floor() + 3;
        let x_coord = 2;
        rock.add_x(x_coord);
        rock.add_y(y_coord);
    }

    /// Get y-coordinate of the highest object (or the floor).
    fn highest_rock_or_floor(&self) -> i32 {
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
            for x in 0..=max_x {
                if all_points.contains(&Point { x, y }) {
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