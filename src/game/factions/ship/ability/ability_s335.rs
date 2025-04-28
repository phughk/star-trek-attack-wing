use crate::game::factions::ship::ability::ability_s337::AbilityS337;
use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS335 {}

impl ShipAbility for AbilityS335 {
    fn get_description(&self) -> &'static str {
        "All Vulcan Captains, Admirals, and [crew] Upgrades equipped to this ship cost -1 SP.\n<b>WHEN THIS SHIP PERFORMS A [scan] ACTION:</b> \nPlace an additional [scan] Token beside this ship."
    }
}