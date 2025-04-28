use crate::game::factions::ship::ability::ability_s271::AbilityS271;
use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS344{}
impl ShipAbility for AbilityS344 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 5+).\n\nCard and model did not come with Star Trek Alliance."
    }
}