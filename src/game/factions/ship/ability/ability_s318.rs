use crate::game::factions::ship::ability::ability_s319::AbilityS319;
use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS318 {}

impl ShipAbility for AbilityS318 {
    fn get_description(&self) -> &'static str {
        "You do not pay a faction penalty for cards equipped to this ship.\n-----------------------------------\nAll [tech] Upgrades equipped to this ship cost -1 SP."
    }
}