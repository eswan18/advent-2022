use crate::valve::Valve;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const STARTING_VALVE: &str = "AA";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistanceMatrix {
    matrix: HashMap<String, HashMap<String, usize>>,
    pub valves: HashMap<String, Valve>,
}

impl DistanceMatrix {
    pub fn new(valves: HashMap<String, Valve>) -> DistanceMatrix {
        let matrix = Self::compute_distance_matrix(&valves);
        DistanceMatrix { matrix, valves }
    }

    fn compute_distance_matrix(
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

    pub fn distance(&self, start: &str, end: &str) -> Option<usize> {
        self.matrix.get(start).and_then(|m| m.get(end).map(|d| *d))
    }

    /// Create a copy of this distance matrix without valves that have 0 flow rate.
    /// Always leave the start node intact though.
    pub fn with_valves_removed(&self) -> DistanceMatrix {
        let mut valves = self.valves.clone();
        let mut matrix = self.matrix.clone();

        let zero_valves = valves
            .values()
            .filter(|v| v.rate == 0)
            .map(|v| v.name.clone())
            .collect::<HashSet<String>>();
        // Remove from the valves hash.
        for name in &zero_valves {
            if name != STARTING_VALVE {
                valves.remove(name);
            }
        }

        // Remove from the distance matrix.
        for name in &zero_valves {
            if name != STARTING_VALVE {
                matrix.remove(name);
            }
            for (_startpoint, endpoint_distances) in matrix.iter_mut() {
                endpoint_distances.remove(name);
            }
        }

        DistanceMatrix { matrix, valves }
    }

    pub fn flow_at(&self, name: &str) -> usize {
        self.valves[name].rate
    }

    pub fn paths_from(&self, name: &str) -> HashMap<String, usize> {
        self.matrix[name].clone()
    }
}

impl Display for DistanceMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let mut valve_names = self.valves.keys().collect::<Vec<&String>>();
        valve_names.sort();
        for start in &valve_names {
            for end in &valve_names {
                if let Some(distance) = self.distance(start, end) {
                    output.push_str(&format!("{} -> {}: {}\n", start, end, distance));
                }
            }
        }
        write!(f, "{}", output)
    }
}
