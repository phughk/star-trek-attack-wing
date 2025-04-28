use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS273{}
impl ShipAbility for AbilityS273 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 5+).\n\nCard and model did not come with Star Trek Alliance."
    }
}