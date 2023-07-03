use std::{fmt::{Display, Formatter}, collections::HashSet};

use crate::distance_matrix::DistanceMatrix;

const STARTING_VALVE: &str = "AA";

#[derive(Debug, Clone, PartialEq, Eq)]
enum PlayerIntention {
    /// The player is moving to a new valve but hasn't arrived yet.
    Moving(IntendedMove),
    /// The player is turning on a valve.
    TurningOn{valve: String},
    /// The player has no plan right now.
    None,
}

#[derive(Debug, Clone)]
struct PlayerState {
    path: Vec<String>,
    total_flow: usize,
    intention: PlayerIntention,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            path: vec![STARTING_VALVE.to_string()],
            total_flow: 0,
            intention: PlayerIntention::None,
        }
    }

    pub fn take_step(&self, steps_remaining: usize, distance_matrix: &DistanceMatrix) -> PlayerState {
        match &self.intention {
            PlayerIntention::Moving(next_move) => {
                let mut next_move = next_move.clone();
                let mut new_player_state = self.clone();
                next_move.moves_remaining -= 1;
                // If we've arrived at the destination, stop moving and plan to turn on the valve the next turn.
                if next_move.moves_remaining == 0 {
                    new_player_state.intention = PlayerIntention::TurningOn { valve: next_move.destination.clone() };
                } else {
                    new_player_state.intention = PlayerIntention::Moving(next_move);
                }
                new_player_state
            },
            PlayerIntention::TurningOn{valve} => {
                let mut new_player_state = self.clone();
                // If we turn on a valve, we get that valve's flow value for every remaining turn.
                new_player_state.total_flow += steps_remaining * distance_matrix.flow_at(&valve);
                new_player_state.path.push(valve.clone());
                // We've finished turning on the valve, so we're done and have no plan anymore.
                new_player_state.intention = PlayerIntention::None;
                new_player_state
            },
            PlayerIntention::None => panic!("PlayerState::take_step called on player with no intention."),
        }
    }

    pub fn with_new_intention(&self, intention: PlayerIntention) -> PlayerState {
        if self.intention != PlayerIntention::None {
            panic!("PlayerState::with_new_intention called on player with existing intention.");
        }
        let mut new_player_state = self.clone();
        new_player_state.intention = intention;
        new_player_state
    }

    pub fn position(&self) -> &String {
        self.path.last().unwrap()
    }
}

/// An intended move is a move that a player has planned/begun but hasn't finished yet.
#[derive(Debug, Clone, PartialEq, Eq)]
struct IntendedMove {
    destination: String,
    moves_remaining: usize,
}

#[derive(Debug, Clone)]
pub struct GameState {
    players: Vec<PlayerState>,
    steps_remaining: usize,
    distance_matrix: DistanceMatrix,
    flow: usize,
}

impl GameState {
    pub fn new(n_players: usize, distance_matrix: &DistanceMatrix, steps: usize) -> GameState {
        let players = (0..n_players).map(|_| PlayerState::new()).collect();
        GameState {
            players,
            steps_remaining: steps,
            distance_matrix: distance_matrix.clone(),
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
        if self.steps_remaining == 0 {
            return vec![self.clone()];
        }
        // Return if we've visited every valve.
        if self.visited_valves().len() == self.distance_matrix.valves.len() {
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
                        !self.players
                            .iter()
                            .any(|p| {
                                if p.path.contains(destination) {
                                    return true;
                                }
                                match &p.intention {
                                    PlayerIntention::Moving(IntendedMove { destination: d, .. }) if d == destination => {
                                        return true;
                                    },
                                    PlayerIntention::TurningOn{valve} if valve == destination  => {
                                        return true;
                                    },
                                    _ => {},
                                }
                                false
                            })
                    })
                    .collect();
                let potential_game_states: Vec<GameState> = potential_steps
                    .into_iter()
                    .map(|(destination, distance)| {
                        let mut new_game_state = self.clone();
                        new_game_state.players[i] = new_game_state.players[i].with_new_intention(PlayerIntention::Moving(IntendedMove {
                            destination,
                            moves_remaining: distance,
                        }));
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
        let mut new_game_state = self.clone();
        new_game_state.steps_remaining -= 1;
        new_game_state.players = new_game_state.players
            .iter()
            .map(|p| p.take_step(new_game_state.steps_remaining, &new_game_state.distance_matrix))
            .collect();
        // Update the game state's flow count.
        new_game_state.flow = new_game_state.players.iter().map(|p| p.total_flow).sum();
        return new_game_state.all_flows();
    }

    fn visited_valves(&self) -> HashSet<&String> {
        self
            .players
            .iter()
            .map(|p| &p.path)
            .flatten()
            .collect()
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameState {{\n")?;
        write!(f, "  players: {:?}\n", self.players)?;
        write!(f, "  steps_remaining: {}\n", self.steps_remaining)?;
        write!(f, "}}")
    }
}