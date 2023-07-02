use crate::grid::{Grid, Position};
use crate::shape::{Diamond, Line, Square};

const MAX_COORD: i32 = 4000000;

pub fn main(contents: String) -> Result<String, String> {
    let grid = Grid::build_from_text(&contents)?;
    // println!("Grid {}", grid);
    let diamonds = Diamond::build_from_grid(grid);
    let perimeter_lines = diamonds
        .iter()
        .flat_map(|d| d.frame())
        .collect::<Vec<Line>>();
    println!("Calculating intersections");
    let mut intersections = Vec::new();
    for i in 0..perimeter_lines.len() {
        let l1 = &perimeter_lines[i];
        for j in i + 1..perimeter_lines.len() {
            let l2 = &perimeter_lines[j];
            if let Some(intersection) = l1.intersection(l2) {
                intersections.push(intersection);
            }
        }
    }
    println!("intersections: {}", intersections.len());

    // Dedup
    println!("Deduping");
    let mut intersections = intersections;
    intersections.sort();
    intersections.dedup();
    println!("intersections: {}", intersections.len());

    println!("Filtering on presence in square.");
    // Filter to just points in the area of interest.
    let area_of_interest = Square::new(0, 0, MAX_COORD, MAX_COORD);
    let intersections = intersections
        .into_iter()
        .filter(|p| area_of_interest.contains(p))
        .collect::<Vec<Position>>();
    println!("intersections: {}", intersections.len());

    println!("Filtering on exclusion from diamonds.");
    // Filter to just points that occur in one of the diamonds.
    let intersections = intersections
        .into_iter()
        .filter(|p| diamonds.iter().all(|d| !d.contains(p)))
        .collect::<Vec<Position>>();
    println!("intersections: {}", intersections.len());

    if intersections.len() != 1 {
        return Err(format!(
            "Expected 1 intersection, found {}",
            intersections.len()
        ));
    }

    let answer = intersections[0].clone();
    // Need to avoid overflows.
    let x = answer.x as i64;
    let y = answer.y as i64;
    let tuning_freq = x * 4000000 + y;
    Ok(tuning_freq.to_string())
}
