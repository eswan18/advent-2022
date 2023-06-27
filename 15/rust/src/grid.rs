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
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sensors = self.sensors();
        let beacons = self.beacons();
        let min_x = sensors.iter().chain(beacons.iter()).map(|p| p.x).min().unwrap();
        let max_x = sensors.iter().chain(beacons.iter()).map(|p| p.x).max().unwrap();
        let min_y = sensors.iter().chain(beacons.iter()).map(|p| p.y).min().unwrap();
        let max_y = sensors.iter().chain(beacons.iter()).map(|p| p.y).max().unwrap();        

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let sensor = sensors.iter().find(|p| p.x == x && p.y == y);
                let beacon = beacons.iter().find(|p| p.x == x && p.y == y);
                match (sensor, beacon) {
                    (Some(_), Some(_)) => write!(f, "X")?,
                    (Some(_), None) => write!(f, "S")?,
                    (None, Some(_)) => write!(f, "B")?,
                    (None, None) => write!(f, ".")?,
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