use crate::distance_matrix::DistanceMatrix;
use crate::player_state::{PlayerState, PlayerIntention, IntendedMove};
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
        all_flows.last().unwrap().flow
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
        for i in 0..self.players.len() {
            if self.players[i].intention == PlayerIntention::None {
                // Find all valves that we can move to and haven't visited yet and aren't in the process of visiting.
                let potential_steps: Vec<(String, usize)> = self
                    .distance_matrix
                    .paths_from(&self.players[i].position())
                    .into_iter()
                    .filter(|(destination, _)| {
                        !self
                            .players
                            .iter()
                            .any(|p| p.owned_valves().contains(destination))
                    })
                    .collect();
                let potential_game_states: Vec<GameState> = potential_steps
                    .into_iter()
                    .map(|(destination, distance)| {
                        let mut new_game_state = self.clone();
                        new_game_state.players[i] = new_game_state.players[i].with_new_intention(
                            PlayerIntention::Moving(IntendedMove {
                                destination,
                                moves_remaining: distance,
                            }),
                        );
                        new_game_state
                    })
                    .collect();
                return potential_game_states
                    .into_iter()
                    .flat_map(|gs| gs.all_flows())
                    .collect();
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
            .map(|p| {
                p.take_step(
                    new_game_state.steps_remaining,
                    &new_game_state.distance_matrix,
                )
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
