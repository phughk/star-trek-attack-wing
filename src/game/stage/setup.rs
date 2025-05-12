use crate::game::factions::StawFaction;
use crate::game::stage::build::BuildState;

/// Engine interface for setting up the game
/// Can customise players, rules, map, sets, points, draft mode etc
pub struct GameSetup {
    pub players: Vec<Player>,
    pub points: Option<u16>,
}

impl GameSetup {
    pub fn new() -> GameSetup {
        GameSetup {
            players: Vec::new(),
            points: Some(100),
        }
    }
}

pub struct Player {
    pub name: String,
    pub picked_factions: Vec<StawFaction>,
}