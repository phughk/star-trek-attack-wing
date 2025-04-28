use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS197{}
impl ShipAbility for AbilityS197 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION (once/game):</b> Every ship at Range 1 sustains 1 damage (including this ship). During the Planning Phase, this ship must choose a \"1\" Maneuver (forward, bank or turn) for the rest of the game."
    }
}