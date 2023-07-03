use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const STARTING_VALVE: &str = "AA";
const STEPS: usize = 30;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct DistanceMatrix {
    matrix: HashMap<String, HashMap<String, usize>>,
    valves: HashMap<String, Valve>,
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

    /// Create a copy ofthis distance matrix without valves that have 0 flow rate.
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

    pub fn maximize_flow(&self) -> usize {
        self.maximize_flow_recursive(STARTING_VALVE, &vec![], 0, 0)
    }

    fn maximize_flow_recursive(&self, at: &str, seen: &Vec<String>, flow: usize, steps_taken: usize) -> usize {
        if seen.len() == self.valves.len() {
            println!("Encountered all valves. Finished path {:?} with flow {}", seen, flow);
            return flow;
        }
        if steps_taken >= STEPS {
            println!("Ran out of steps. Finished path {:?} with flow {}", seen, flow);
            return flow;
        }
        let mut potential_steps: Vec<(String, usize)> = self
            .paths_from(&at)
            .into_iter()
            .filter(|(name, _)| !seen.contains(name))
            .collect();
        if potential_steps.len() == 0 {
            println!("Hit dead end. Finished path {:?} with flow {}", seen, flow);
            return flow;
        }
        // Sort by distance, ascending.
        potential_steps.sort_by(|(_, a), (_, b)| a.cmp(b));
        let mut total_flows = vec![];
        for (destination, distance) in potential_steps {
            let mut seen = seen.clone();
            seen.push(destination.clone());
            // Account for both the distance traveled and the time spent turning on the valve.
            let steps_taken = steps_taken + distance + 1;
            if steps_taken > STEPS {
                println!("Ran out of steps. Finished path {:?} with flow {}", seen, flow);
                return flow;
            }
            let steps_remaining = STEPS - steps_taken;
            let flow = flow + self.flow_at(&destination) * steps_remaining;
            let total_flow = self.maximize_flow_recursive(&destination, &seen, flow, steps_taken);
            total_flows.push(total_flow);
        }
        match total_flows.into_iter().max() {
            Some(max) => max,
            None => panic!("No flows found"),
        }
    }

    fn flow_at(&self, name: &str) -> usize {
        self.valves[name].rate
    }

    fn paths_from(&self, name: &str) -> HashMap<String, usize> {
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
