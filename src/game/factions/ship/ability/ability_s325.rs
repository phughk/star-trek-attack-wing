use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS325{}

impl ShipAbility for AbilityS325 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Target a friendly ship within Range 1-2.\n\nRepair 1 Shield and 1 Hull on the target ship."
    }
}