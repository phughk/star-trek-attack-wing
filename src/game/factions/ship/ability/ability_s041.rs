use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS041{}
impl ShipAbility for AbilityS041 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 8+).\n\nCard and model did not come with Star Trek Alliance."
    }
}