use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS331 {}

impl ShipAbility for AbilityS331 {
    fn get_description(&self) -> &'static str {
        "All friendly Dominion ships within Range 1 roll +1 attack die."
    }
}