use std::cmp;
use std::fmt::Display;
use std::collections::HashSet;

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
        let sensors = self.sensors();
        let beacons = self.beacons();
        let excluded = self.excluded_positions();
        let excluded: HashSet<&Position> = excluded.iter().collect();
        let all_positions = sensors.iter().chain(beacons.iter()).chain(excluded.iter());

        let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
        for p in all_positions {
            if p.x < min_x {
                min_x = p.x
            }
            if p.x > max_x {
                max_x = p.x
            }
            if p.y < min_y {
                min_y = p.y
            }
            if p.y > max_y {
                max_y = p.y
            }
        }
        (Position{ x: min_x, y: min_y }, Position{ x: max_x, y: max_y })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sensors = self.sensors();
        let beacons = self.beacons();
        let excluded = self.excluded_positions();
        let (Position{ x: min_x, y: min_y }, Position{ x: max_x, y: max_y }) = self.min_and_max_coords();
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
            print!("{: >8}", y);
            for x in min_x..=max_x {
                let position = Position{ x, y };
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
        let mut readings = Vec::new();
        for line in text.lines() {
            let reading = Reading::build_from_line(line)?;
            readings.push(reading);
        }
        Ok(Grid { readings })
    }

    fn excluded_positions(&self) -> HashSet<Position> {
        self.readings.iter().map(|r| r.excluded_positions()).flatten().collect()
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
        Reading { sensor, beacon, distance }
    }

    // Get all the positions that we can rule out as having a beacon.
    pub fn excluded_positions(&self) -> HashSet<Position> {
        let mut excluded = HashSet::new();
        let x_range = self.sensor.x - self.distance..=self.sensor.x + self.distance;
        let y_range = self.sensor.y - self.distance..=self.sensor.y + self.distance;
        for x in x_range {
            for y in y_range.clone() {
                let p = Position{ x, y };
                if manhattan(&p, &self.sensor) <= self.distance {
                    excluded.insert(Position { x, y });
                }
            }
        }
        excluded
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    pub x: i32,
    pub y: i32,
}

impl Reading {
    pub fn build_from_line(text: &str) -> Result<Reading, String> {
        // Readings look like: "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        let text = text.trim();
        println!("text: {}", text);
        let re = Regex::new(r"x=(-?[\d]+), y=(-?[\d]+).*x=([-?\d]+), y=(-?[\d]+)").unwrap();
        let captures = re.captures(text).ok_or(format!("didn't match text: '{}'", text))?;
        let sensor_x = captures.get(1).unwrap().as_str().parse::<i32>().map_err(|_| "bad parse")?;
        let sensor_y = captures.get(2).unwrap().as_str().parse::<i32>().map_err(|_| "bad parse")?;
        let beacon_x = captures.get(3).unwrap().as_str().parse::<i32>().map_err(|_| "bad parse")?;
        let beacon_y = captures.get(4).unwrap().as_str().parse::<i32>().map_err(|_| "bad parse")?;
        Ok(Reading::new(
            Position { x: sensor_x, y: sensor_y },
            Position { x: beacon_x, y: beacon_y },
        ))
    }
}