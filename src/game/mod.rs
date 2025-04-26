use crate::game::config::StawGameConfig;

mod config;
mod state;
mod factions;

pub struct StawGame {

}

impl StawGame {
    pub fn new(config: StawGameConfig) -> Result<Self, &'static str> {
        if config.players.len()<2 {
            return Err("Not enough players");
        }
        Ok(StawGame {

        })
    }
}
