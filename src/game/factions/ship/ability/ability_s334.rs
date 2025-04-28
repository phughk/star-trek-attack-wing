use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS334{}

impl ShipAbility for AbilityS334 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> \nAll other friendly Dominion ships within Range 1-2 may perform an Action on their Action Bar as a Free Action."
    }
}