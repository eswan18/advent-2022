use std::collections::HashSet;
use crate::rock::{Rock, RockDropper};
use crate::point::Point;

const LOG: bool = false;
const CHAMBER_WIDTH: i32 = 7;

#[derive(Clone)]
pub struct Chamber {
    set_rocks: Vec<Rock>,
    rock_dropper: RockDropper,
    jet_pattern: Vec<char>,
    jet_count: usize,
}

impl Chamber {
    pub fn new(jet_pattern: &str) -> Chamber {
        let set_rocks = vec![];
        let jet_pattern = jet_pattern.chars().collect();
        let rock_dropper = RockDropper::new_with_default();
        Chamber { set_rocks, rock_dropper, jet_pattern, jet_count: 0 }
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
        if LOG {
            let mut fake_chamber = self.clone();
            fake_chamber.set_rocks.push(rock.clone());
            println!("{}", fake_chamber);
        }
        loop {
            // Let the rock be pushed by a jet. If it's obstructed, we don't really care.
            let _ = self.push_jet(&mut rock);
            if LOG {
                let mut fake_chamber = self.clone();
                fake_chamber.set_rocks.push(rock.clone());
                println!("{}", fake_chamber);
            }
            // Let the rock fall one square.
            let stuck = self.push_down(&mut rock).is_err();
            if LOG {
                let mut fake_chamber = self.clone();
                fake_chamber.set_rocks.push(rock.clone());
                println!("{}", fake_chamber);
            }
            // If it can't fall any more, we're done and can move on.
            if stuck {
                break;
            }
        }
        self.set_rocks.push(rock);
        if LOG {
            println!("{}", self);
        }
    }

    /// Push the rock one square in the direction of the jet if it isn't obstructed.
    fn push_jet(&mut self, rock: &mut Rock) -> Result<(), String> {
        let index = self.jet_count % self.jet_pattern.len();
        let jet = self.jet_pattern[index];
        self.jet_count += 1;
        match jet {
            '<' => {
                if self.is_obstructed_left(&rock) {
                    if LOG {
                        println!("left obstruction!");
                    }
                    return Err(format!("Rock {:?} is obstructed to the left", rock))
                }
                rock.add_x(-1);
            }
            '>' => {
                if self.is_obstructed_right(&rock) {
                    if LOG {
                        println!("right obstruction!");
                    }
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

    fn is_obstructed_below(&self, rock: &Rock) -> bool {
        let mut all_points = self.all_points();
        // Add the floor to the set of points.
        for x in 0..CHAMBER_WIDTH {
            all_points.insert(Point { x, y: -1 });
        }
        rock.points.iter().any(|p| all_points.contains(&Point { x: p.x, y: p.y-1 }))
    }

    fn is_obstructed_left(&self, rock: &Rock) -> bool {
        let mut all_points = self.all_points();
        // Add the right wall to the set of points.
        for y in 0..=(self.highest_point() + 5) {
            all_points.insert(Point { x: -1, y });
        }
        rock.points.iter().any(|p| all_points.contains(&Point { x: p.x-1, y: p.y }))
    }

    fn is_obstructed_right(&self, rock: &Rock) -> bool {
        let mut all_points = self.all_points();
        // Add the right wall to the set of points.
        for y in 0..=(self.highest_point() + 5) {
            all_points.insert(Point { x: CHAMBER_WIDTH, y });
        }
        rock.points.iter().any(|p| all_points.contains(&Point { x: p.x+1, y: p.y }))
    }

    /// Takes a newly-dropped rock and puts it in the right place in the chamber: 2 units from the left edge,
    /// and 3 units above the highest rock or the floor.
    fn align_dropped_rock(&self, rock: &mut Rock) {
        let y_coord = self.highest_point() + 4;
        let x_coord = 2;
        rock.add_x(x_coord);
        rock.add_y(y_coord);
    }

    /// Get y-coordinate of the highest object (or the floor).
    pub fn highest_point(&self) -> i32 {
        let all_points = self.all_points();
        // The floor is at -1, so that's the default if there are no rocks.
        all_points.iter().map(|p| p.y).max().unwrap_or(-1)
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all_points = self.all_points();
        let max_y = all_points.iter().map(|p| p.y).max().unwrap_or(0);
        for y in 0..=max_y {
            // Adjust for the fact that we're printing from the top, but the points are relative to the bottom.
            let y = max_y - y;
            write!(f, "|")?;
            for x in 0..CHAMBER_WIDTH {
                if all_points.contains(&Point { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "--FLOOR--")?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_obstructed_right() {
        let chamber = Chamber::new(">");
        let mut rock = Rock::build_from_text("#");
        // Align the rock, putting it at (2, 3).
        chamber.align_dropped_rock(&mut rock);
        // The rock shouldn't be obstructed.
        assert!(!chamber.is_obstructed_right(&rock));
        // Push to the right, putting it at (3, 3).
        rock.add_x(1);
        assert!(!chamber.is_obstructed_right(&rock));
        // Push to the right, putting it at (4, 3).
        rock.add_x(1);
        assert!(!chamber.is_obstructed_right(&rock));
        // Push to the right, putting it at (5, 3).
        rock.add_x(1);
        assert!(!chamber.is_obstructed_right(&rock));
        // Push to the right, putting it at (6, 3). Now it should be obstructed.
        rock.add_x(1);
        assert!(chamber.is_obstructed_right(&rock));
    }

    #[test]
    fn test_drop_rock() {
        let mut chamber = Chamber::new(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let mut expected_points: HashSet<Point> = HashSet::new();

        chamber.drop_rock();
        let new_points = vec![(2, 0), (3, 0), (4, 0), (5, 0)];
        expected_points.extend(new_points.iter().map(|(x, y)| Point { x: *x, y: *y }));
        assert_eq!(chamber.all_points(), expected_points);

        chamber.drop_rock();
        let new_points = vec![(3, 1), (2, 2), (3, 2), (4, 2), (3, 3)];
        expected_points.extend(new_points.iter().map(|(x, y)| Point { x: *x, y: *y }));
        assert_eq!(chamber.all_points(), expected_points);

        chamber.drop_rock();
        let new_points = vec![(0, 3), (1, 3), (2, 3), (2, 4), (2, 5)];
        expected_points.extend(new_points.iter().map(|(x, y)| Point { x: *x, y: *y }));
        assert_eq!(chamber.all_points(), expected_points);
    }
}