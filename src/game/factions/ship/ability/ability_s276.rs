use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS276{}

impl ShipAbility for AbilityS276 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Target a friendly ship within Range 1-2.\n\nRepair 1 Shield or 1 Hull on the target ship."
    }
}