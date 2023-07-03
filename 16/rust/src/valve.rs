use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const STARTING_VALVE: &str = "AA";

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

#[derive(Debug, Clone)]
struct Path {
    valves: Vec<String>,
    flow: usize,
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

    pub fn maximize_flow(&self, max_steps: usize) -> usize {
        let mut all_flows = self.all_flows(Path{valves: vec![], flow: 0}, 0, max_steps);
        all_flows.sort_by(|p1, p2| p1.flow.cmp(&p2.flow));
        all_flows.last().unwrap().flow
    }

    pub fn maximize_flow_2_paths(&self, max_steps: usize) -> usize {
        let mut all_flows = self.all_flows_2_paths(Path{valves: vec![], flow: 0}, Path{valves: vec![], flow: 0}, 0, max_steps);
        all_flows.sort_by(|p1, p2| p1.flow.cmp(&p2.flow));
        all_flows.last().unwrap().flow
    }

    fn all_flows(&self, path: Path, steps_taken: usize, max_steps: usize) -> Vec<Path> {
        if path.valves.len() == self.valves.len() {
            println!("Encountered all valves. Finished path {:?} with flow {}", path.valves, path.flow);
            return vec![path];
        }
        if steps_taken >= max_steps {
            println!("Ran out of steps. Finished path {:?} with flow {}", path.valves, path.flow);
            return vec![path];
        }
        let at = path.valves.last().map(|s| s.as_str()).unwrap_or(STARTING_VALVE);
        let potential_steps: Vec<(String, usize)> = self
            .paths_from(&at)
            .into_iter()
            .filter(|(name, _)| !path.valves.contains(name))
            .collect();
        if potential_steps.len() == 0 {
            println!("Hit dead end. Finished path {:?} with flow {}", path.valves, path.flow);
            return vec![path];
        }
        potential_steps
            .into_iter()
            .map(|(destination, distance)| {
                let mut seen = path.valves.clone();
                seen.push(destination.clone());
                // Account for both the distance traveled and the time spent turning on the valve.
                let steps_taken = steps_taken + distance + 1;
                if steps_taken > max_steps {
                    return vec![Path{valves: seen, flow: path.flow}];
                }
                let steps_remaining = max_steps - steps_taken;
                let flow = path.flow + self.flow_at(&destination) * steps_remaining;
                let new_path = Path{valves: seen, flow};
                self.all_flows(new_path, steps_taken, max_steps)
            })
            .flatten()
            .collect() 
    }

    fn all_flows_2_paths(&self, path_1: Path, path_2: Path, steps_taken: usize, max_steps: usize) -> Vec<(Path, Path)> {
        if path_1.valves.len() + path_2.valves.len() == self.valves.len() {
            println!("Encountered all valves. Finished paths {:?} with flow {}", (path_1.valves, path_2.valves), path_1.flow + path_2.flow);
            return vec![(path_1, path_2)];
        }
        if steps_taken >= max_steps {
            println!("Ran out of steps. Finished paths {:?} with flow {}", (path_1.valves, path_2.valves), path_1.flow + path_2.flow);
            return vec![(path_1, path_2)];
        }
        let at_1 = path_1.valves.last().map(|s| s.as_str()).unwrap_or(STARTING_VALVE);
        let at_2 = path_2.valves.last().map(|s| s.as_str()).unwrap_or(STARTING_VALVE);

        let potential_steps_1: Vec<(String, usize)> = self
            .paths_from(&at_1)
            .into_iter()
            .filter(|(name, _)| !path_1.valves.contains(name) && !path_2.valves.contains(name))
            .collect();
        let potential_steps_2: Vec<(String, usize)> = self
            .paths_from(&at_2)
            .into_iter()
            .filter(|(name, _)| !path_1.valves.contains(name) && !path_2.valves.contains(name))
            .collect();
        if potential_steps_1.len() == 0 && potential_steps_2.len() == 0 {
            println!("Hit dead end. Finished paths {:?} with flow {}", (path_1.valves, path_2.valves), path_1.flow + path_2.flow);
            return vec![(path_1, path_2)];
        }
        if potential_steps_1.len() == 0 {
            // If player 1 is stuck, only move player 2.
            return potential_steps_2
                .into_iter()
                .map(|(destination, distance)| {
                    let mut seen = path_2.valves.clone();
                    seen.push(destination.clone());
                    // Account for both the distance traveled and the time spent turning on the valve.
                    let steps_taken = steps_taken + distance + 1;
                    if steps_taken > max_steps {
                        return vec![(path_1.clone(), Path{valves: seen, flow: path.flow})];
                    }
                    let steps_remaining = max_steps - steps_taken;
                    let flow = path_2.flow + self.flow_at(&destination) * steps_remaining;
                    let new_path = Path{valves: seen, flow};
                    self.all_flows_2_paths(path_1.clone(), new_path, steps_taken, max_steps)
                })
                .flatten()
                .collect() 
        } else if potential_steps_2.len() == 0 {
            // If player 2 is stuck, only move player 1.
            return potential_steps_1
                .into_iter()
                .map(|(destination, distance)| {
                    let mut seen = path_1.valves.clone();
                    seen.push(destination.clone());
                    // Account for both the distance traveled and the time spent turning on the valve.
                    let steps_taken = steps_taken + distance + 1;
                    if steps_taken > max_steps {
                        return vec![(Path{valves: seen, flow: path.flow}, path_2.clone())];
                    }
                    let steps_remaining = max_steps - steps_taken;
                    let flow = path_1.flow + self.flow_at(&destination) * steps_remaining;
                    let new_path = Path{valves: seen, flow};
                    self.all_flows_2_paths(new_path, path_2.clone(), steps_taken, max_steps)
                })
                .flatten()
                .collect();
        } 
        // If there are potential steps for both players, find all combinations of them.
        for (destination_1, distance_1) in potential_steps_1 {
            for (destination_2, distance_2) in potential_steps_2 {
                let mut seen_1 = path_1.valves.clone();
                seen_1.push(step_1.0.clone());
                let mut seen_2 = path_2.valves.clone();
                seen_2.push(step_2.0.clone());
                // Account for both the distance traveled and the time spent turning on the valve.
                let steps_taken = steps_taken + step_1.1 + step_2.1 + 1;
                if steps_taken > max_steps {
                    return vec![(Path{valves: seen_1, flow: path_1.flow}, Path{valves: seen_2, flow: path_2.flow})];
                }
                let steps_remaining = max_steps - steps_taken;
                let flow_1 = path_1.flow + self.flow_at(&step_1.0) * steps_remaining;
                let flow_2 = path_2.flow + self.flow_at(&step_2.0) * steps_remaining;
                let new_path_1 = Path{valves: seen_1, flow: flow_1};
                let new_path_2 = Path{valves: seen_2, flow: flow_2};
                return self.all_flows_2_paths(new_path_1, new_path_2, steps_taken, max_steps);
            }
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
