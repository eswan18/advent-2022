use std::collections::{HashMap, HashSet};

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

pub fn compute_distance_matrix(
    valves: &HashMap<String, Valve>,
) -> HashMap<String, HashMap<String, usize>> {
    // e.g. matrix[a][b] = 3 means that fastest route from a to b is 3
    let mut distance_matrix: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let all_valve_names: Vec<String> = valves.keys().map(|s| s.clone()).collect();
    // Find the shortest path from each valve to each other valve.
    for start in &all_valve_names {
        // Start having seen only the start node. Track the distance from the start node to each.
        let mut seen: HashMap<String, usize> = HashMap::new();
        let mut distance = 0;
        seen.insert(start.clone(), distance);
        // Loop until we've seen every endpoint (excluding the start).
        while seen.len() < all_valve_names.len() {
            distance += 1;
            // For each node we've seen, find all the nodes we haven't seen that it's connected to.
            let seen_copy = seen.clone();
            let seen_to_this_point: Vec<&String> = seen_copy.keys().collect();
            for name in &seen_to_this_point {
                let valve = &valves[*name];
                valve.linked_valves.iter().for_each(|v| { 
                    if !seen_to_this_point.contains(&v) {
                        seen.insert(v.to_string(), distance);
                    }
                });
            }
        }
        // Remove the starting node -- it's redundant.
        seen.remove(start);
        distance_matrix.insert(start.clone(), seen);
    }
    distance_matrix
}
