use crate::range::Range;

pub fn main(input_path: &str) -> Result<i32, String> {
    let contents = match std::fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Error reading file: {}", e)),
    };
    let lines = contents.lines();

    let range_pairs: Vec<Result<(Range, Range), String>> = lines.map(Range::from_line).collect();
    let range_pairs: Vec<(Range, Range)> = range_pairs
        .into_iter()
        .map(|r| match r {
            Ok(r) => r,
            Err(e) => panic!("Error: {}", e),
        })
        .collect();
    
    let contains_count: i32 = range_pairs
        .iter()
        .map(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
        .map(|x| x as i32)
        .sum();
    
    Result::Ok(contains_count)
}
