use std::collections::HashSet;
use std::fmt::Display;

use regex::Regex;

fn manhattan(p1: &Position, p2: &Position) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

pub struct Grid {
    readings: Vec<Reading>,
}

impl Grid {
    fn sensors(&self) -> HashSet<&Position> {
        self.readings.iter().map(|r| &r.sensor).collect()
    }

    fn beacons(&self) -> HashSet<&Position> {
        self.readings.iter().map(|r| &r.beacon).collect()
    }

    fn min_and_max_coords(&self) -> (Position, Position) {
        let readings = &self.readings;
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for r in readings {
            let r_min_x = r.sensor.x - r.distance;
            let r_max_x = r.sensor.x + r.distance;
            let r_min_y = r.sensor.y - r.distance;
            let r_max_y = r.sensor.y + r.distance;
            if r_min_x < min_x {
                min_x = r_min_x;
            }
            if r_max_x > max_x {
                max_x = r_max_x;
            }
            if r_min_y < min_y {
                min_y = r_min_y;
            }
            if r_max_y > max_y {
                max_y = r_max_y;
            }
        }
        (Position{ x: min_x, y: min_y }, Position{ x: max_x, y: max_y })
    }

    fn greatest_reading_distance(&self) -> i32 {
        self.readings.iter().map(|r| r.distance).max().unwrap()
    }

    pub fn excluded_count_in_row(&self, y: i32) -> i32 {
        // A more performant version of the above function.
        // Start at the left-most point plus the greatest distance of any reading, and go to the
        // right-most point plus the greatest distance of any reading.
        // For each point, check if it's excluded based on being near enough to a sensor or beacon.
        let (Position { x: min_x, .. }, Position{ x: max_x, .. }) = self.min_and_max_coords();
        let max_distance = self.greatest_reading_distance();
        let min_x = min_x - max_distance;
        let max_x = max_x + max_distance;
        let mut excluded = vec![];
        'x_loop: for x in min_x..=max_x {
            let position = Position{ x, y };
            for r in &self.readings {
                if manhattan(&position, &r.sensor) <= r.distance {
                    // This position is within range of a sensor, so it's excluded.
                    excluded.push(position);
                    continue 'x_loop;
                }
            }
        }
        // Go back through and remove positions that are occupied by a sensor or beacon.
        let objects = self.sensors().into_iter().chain(self.beacons().into_iter()).collect::<HashSet<&Position>>();
        excluded = excluded.into_iter().filter(|e| !objects.contains(e)).collect();
        excluded.len() as i32
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sensors = self.sensors();
        let beacons = self.beacons();
        let excluded = self.excluded_positions();
        let (Position { x: min_x, y: min_y }, Position { x: max_x, y: max_y }) =
            self.min_and_max_coords();
        // Print our numbers across the top before starting the loop proper.
        write!(f, "        ")?;
        for x in min_x..=max_y {
            if x <= 9 && x >= 0 {
                write!(f, "{}", x)?;
            } else {
                write!(f, " ")?;
            }
        }
        write!(f, "\n")?;
        for y in min_y..=max_y {
            write!(f, "{: >8}", y)?;
            for x in min_x..=max_x {
                let position = Position { x, y };
                let sensor = sensors.contains(&position);
                let beacon = beacons.contains(&position);
                let excluded = excluded.contains(&position);
                if sensor {
                    write!(f, "S")?;
                } else if beacon {
                    write!(f, "B")?;
                } else if excluded {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Grid {
    pub fn build_from_text(text: &str) -> Result<Grid, String> {
        println!("Parsing...");
        let mut readings = Vec::new();
        for line in text.lines() {
            let reading = Reading::build_from_line(line)?;
            readings.push(reading);
        }
        println!("Done parsing.");
        Ok(Grid { readings })
    }

    fn excluded_positions(&self) -> HashSet<Position> {
        println!("Grid.excluded_positions ->");
        let e = self
            .readings
            .iter()
            .map(|r| r.excluded_positions())
            .flatten()
            .collect();
        println!("Done");
        e
    }
    
    pub fn open_spaces_with_max_coord(&self, max_coord: i32) -> Vec<Position> {
        let mut open_spaces = Vec::new();
        for x in 0..=max_coord {
            if x % 100 == 0 {
                println!("Checking x = {}", x);
            }
            for y in 0..=max_coord {
                let position = Position { x, y };
                if self.is_open_space(&position) {
                    println!("Found open space at {:?}", position);
                    open_spaces.push(position);
                }
            }
        }
        open_spaces
    }

    fn is_open_space(&self, position: &Position) -> bool {
        let readings = &self.readings;
        for r in readings {
            if manhattan(&r.sensor, position) <= r.distance {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Reading {
    sensor: Position,
    // The closest beacon to this sensor.
    beacon: Position,
    // The manhattan distance between them.
    distance: i32,
}

impl Reading {
    pub fn new(sensor: Position, beacon: Position) -> Reading {
        let distance = manhattan(&sensor, &beacon);
        Reading {
            sensor,
            beacon,
            distance,
        }
    }

    // Get all the positions that we can rule out as having a beacon.
    pub fn excluded_positions(&self) -> HashSet<Position> {
        println!("Reading.excluded_positions ->");
        let mut excluded = HashSet::new();
        let x_range = self.sensor.x - self.distance..=self.sensor.x + self.distance;
        let y_range = self.sensor.y - self.distance..=self.sensor.y + self.distance;
        for x in x_range {
            for y in y_range.clone() {
                let p = Position { x, y };
                if manhattan(&p, &self.sensor) <= self.distance {
                    excluded.insert(Position { x, y });
                }
            }
        }
        println!("Done.");
        excluded
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Reading {
    pub fn build_from_line(text: &str) -> Result<Reading, String> {
        // Readings look like: "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        let text = text.trim();
        let re = Regex::new(r"x=(-?[\d]+), y=(-?[\d]+).*x=([-?\d]+), y=(-?[\d]+)").unwrap();
        let captures = re
            .captures(text)
            .ok_or(format!("didn't match text: '{}'", text))?;
        let sensor_x = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| "bad parse")?;
        let sensor_y = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| "bad parse")?;
        let beacon_x = captures
            .get(3)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| "bad parse")?;
        let beacon_y = captures
            .get(4)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| "bad parse")?;
        Ok(Reading::new(
            Position {
                x: sensor_x,
                y: sensor_y,
            },
            Position {
                x: beacon_x,
                y: beacon_y,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_excluded_count_in_row_fast() {
        let text = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let grid = Grid::build_from_text(text).unwrap();
        assert_eq!(26, grid.excluded_count_in_row(10));
    }

    #[test]
    fn test_min_and_max_coords_fast() {
        let text = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let grid = Grid::build_from_text(text).unwrap();
        let (p1, p2) = grid.min_and_max_coords();
        assert_eq!(Position { x: -8, y: -10 }, p1);
        assert_eq!(Position { x: 28, y: 26 }, p2);
    }
}
