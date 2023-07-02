#[derive(Debug)]
pub struct Valve {
    pub name: String,
    rate: usize,
    linked_valves: Vec<String>,
}

impl Valve {
    pub fn new(name: String, rate: usize, linked_valves: Vec<&str>) -> Valve {
        Valve {
            name,
            rate,
            linked_valves: linked_valves.iter().map(|s| String::from(*s)).collect(),
        }
    }
}