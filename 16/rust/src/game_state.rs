use crate::distance_matrix::DistanceMatrix;
use crate::player_state::{IntendedMove, PlayerIntention, PlayerState};
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    players: Vec<PlayerState>,
    steps_remaining: usize,
    distance_matrix: Rc<DistanceMatrix>,
    flow: usize,
}

impl GameState {
    pub fn new(n_players: usize, distance_matrix: Rc<DistanceMatrix>, steps: usize) -> GameState {
        let players = (0..n_players).map(|_| PlayerState::new()).collect();
        GameState {
            players,
            steps_remaining: steps,
            distance_matrix: distance_matrix,
            flow: 0,
        }
    }

    pub fn maximize_flow(&self) -> usize {
        let mut all_flows = self.all_flows();
        all_flows.sort_by(|a, b| a.flow.cmp(&b.flow));
        let max_flow = all_flows.last().unwrap();
        println!("{}", max_flow);
        max_flow.flow
    }

    pub fn all_flows(&self) -> Vec<GameState> {
        // Return if we've run out of steps.
        if self.steps_remaining <= 0 {
            return vec![self.clone()];
        }
        // Return if we've visited every valve.
        if self.enabled_valves().len() == self.distance_matrix.valves.len() {
            return vec![self.clone()];
        }
        // If any player has no intended move or valve to enable, then we can't actually take a step yet.
        // Recurse with every possible move for that player.
        let idle_players: Vec<(usize, &PlayerState)> = self
            .players
            .iter()
            .enumerate()
            .filter(|(_, p)| p.intention == PlayerIntention::None)
            .collect();
        if idle_players.len() > 0 {
            let all_owned_valves: HashSet<String> = self
                .players
                .iter()
                .map(|p| p.owned_valves())
                .flatten()
                .collect();
            let valves_left_to_visit = self
                .distance_matrix
                .valves
                .keys()
                .filter(|valve_name| !all_owned_valves.contains(*valve_name))
                .collect::<Vec<&String>>();
            if valves_left_to_visit.len() > 0 {
                let mut potential_next_states: Vec<GameState> = vec![];
                valves_left_to_visit
                    .iter()
                    .combinations(idle_players.len())
                    .for_each(|valves_to_visit| {
                        // Assign each idle player to a valve.
                        let mut new_game_state = self.clone();
                        let mut valve_iter = valves_to_visit.iter();
                        for (i, p) in idle_players.iter() {
                            let player_position = p.path.last().unwrap();
                            let valve_to_visit = valve_iter.next().unwrap();
                            let distance = new_game_state
                                .distance_matrix
                                .distance(player_position, valve_to_visit)
                                .unwrap();
                            new_game_state.players[*i].intention =
                                PlayerIntention::Moving(IntendedMove {
                                    destination: (**valve_to_visit).clone(),
                                    moves_remaining: distance,
                                });
                        }
                        potential_next_states.push(new_game_state);
                    });
                if potential_next_states.len() > 0 {
                    return potential_next_states
                        .into_iter()
                        .flat_map(|gs| gs.all_flows())
                        .collect();
                }
            }
        }
        // If every player knows what they're doing, just advance them all one step.
        self.take_step().all_flows()
    }

    pub fn take_step(&self) -> GameState {
        let mut new_game_state = self.clone();
        new_game_state.steps_remaining -= 1;
        new_game_state.players = new_game_state
            .players
            .iter()
            .map(|p| match p.intention {
                PlayerIntention::None => p.clone(),
                _ => p.take_step(
                    new_game_state.steps_remaining,
                    &new_game_state.distance_matrix,
                ),
            })
            .collect();
        // Update the game state's flow count.
        new_game_state.flow = new_game_state.players.iter().map(|p| p.total_flow).sum();
        new_game_state
    }

    fn enabled_valves(&self) -> HashSet<&String> {
        self.players.iter().map(|p| &p.path).flatten().collect()
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameState ({} steps remaining)", self.steps_remaining)?;
        for (player_id, path) in self.players.iter().enumerate() {
            write!(f, "\n{}: {}", player_id, path)?;
        }
        Ok(())
    }
}
