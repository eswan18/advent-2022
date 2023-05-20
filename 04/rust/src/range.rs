pub struct Range {
    start: i32,
    end: i32,
}

impl Range {
    pub fn from_line(line: &str) -> Result<(Self, Self), String> {
        let parts: Vec<&str> = line.split(",").collect();
        if parts.len() != 2 {
            panic!("Need two ranges per line");
        }
        Ok((
            Range::from_string(parts[0])?,
            Range::from_string(parts[1])?,
        ))
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let parts: Vec<&str> = s.split("-").collect();
        if parts.len() != 2 {
            return Err(String::from("Need two numbers per range"));
        }
        let parsed_parts: Vec<Result<i32, _>> = parts
            .into_iter()
            .map( |s| s.parse::<i32>())
            .collect();
        let start = match &parsed_parts[0] {
            Ok(n) => n,
            Err(e) => return Err(format!("Error parsing start: {}", e)),
        };
        let end = match &parsed_parts[1] {
            Ok(n) => n,
            Err(e) => return Err(format!("Error parsing end: {}", e)),
        };
        Ok(Range{ start: *start, end: *end })
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}