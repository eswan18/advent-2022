use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::distance_matrix::DistanceMatrix;

const STARTING_VALVE: &str = "AA";


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerIntention {
    /// The player is moving to a new valve but hasn't arrived yet.
    Moving(IntendedMove),
    /// The player is turning on a valve.
    TurningOn { valve: String },
    /// The player has no plan right now.
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerState {
    pub path: Vec<String>,
    pub total_flow: usize,
    pub intention: PlayerIntention,
}

impl PlayerState {
    pub fn new() -> PlayerState {
        PlayerState {
            path: vec![STARTING_VALVE.to_string()],
            total_flow: 0,
            intention: PlayerIntention::None,
        }
    }

    pub fn take_step(
        &self,
        steps_remaining: usize,
        distance_matrix: &DistanceMatrix,
    ) -> PlayerState {
        match &self.intention {
            PlayerIntention::Moving(next_move) => {
                let mut next_move = next_move.clone();
                let mut new_player_state = self.clone();
                next_move.moves_remaining -= 1;
                // If we've arrived at the destination, stop moving and plan to turn on the valve the next turn.
                if next_move.moves_remaining == 0 {
                    new_player_state.intention = PlayerIntention::TurningOn {
                        valve: next_move.destination.clone(),
                    };
                } else {
                    new_player_state.intention = PlayerIntention::Moving(next_move);
                }
                new_player_state
            }
            PlayerIntention::TurningOn { valve } => {
                let mut new_player_state = self.clone();
                // If we turn on a valve, we get that valve's flow value for every remaining turn.
                new_player_state.total_flow += steps_remaining * distance_matrix.flow_at(&valve);
                new_player_state.path.push(valve.clone());
                // We've finished turning on the valve, so we're done and have no plan anymore.
                new_player_state.intention = PlayerIntention::None;
                new_player_state
            }
            PlayerIntention::None => {
                panic!("PlayerState::take_step called on player with no intention.")
            }
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

    pub fn owned_valves(&self) -> HashSet<String> {
        let mut valves: HashSet<String> = self.path.iter().cloned().collect();
        if let PlayerIntention::TurningOn { valve } = &self.intention {
            valves.insert(valve.clone());
        }
        if let PlayerIntention::Moving(next_move) = &self.intention {
            valves.insert(next_move.destination.clone());
        }
        valves
    }
}

/// An intended move is a move that a player has planned/begun but hasn't finished yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntendedMove {
    pub destination: String,
    pub moves_remaining: usize,
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.join("=>"))?;
        match &self.intention {
            PlayerIntention::None => {}
            PlayerIntention::Moving(next_move) => {
                write!(
                    f,
                    " ->{} ({})",
                    &next_move.destination, &next_move.moves_remaining
                )?;
            }
            PlayerIntention::TurningOn { valve } => {
                write!(f, "  @{} (0/1)", &valve)?;
            }
        }
        Ok(())
    }
}