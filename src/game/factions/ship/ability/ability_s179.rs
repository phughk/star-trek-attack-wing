use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS179{}
impl ShipAbility for AbilityS179 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 4+).\n\nCard and model did not come with Star Trek Alliance."
    }
}