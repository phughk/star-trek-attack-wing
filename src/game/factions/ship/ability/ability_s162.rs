use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS162{}
impl ShipAbility for AbilityS162 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 2+).\n\nCard and model did not come with Star Trek Alliance."
    }
}