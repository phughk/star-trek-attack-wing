use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS343{}

impl ShipAbility for AbilityS343 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING:</b> If this ship spends a [battlestations] Token:\n\nPlace a [battlestations] Token beside this ship."
    }
}