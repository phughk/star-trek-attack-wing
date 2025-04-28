use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS090{}
impl ShipAbility for AbilityS090 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 8+).\n\nCard and model did not come with Star Trek Alliance."
    }
}