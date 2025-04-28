use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS125{}
impl ShipAbility for AbilityS125 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 4+).\n\nCard and model did not come with Star Trek Alliance."
    }
}