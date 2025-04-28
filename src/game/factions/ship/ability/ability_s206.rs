use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS206{}
impl ShipAbility for AbilityS206 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 5+).\n\nCard and model did not come with Star Trek Alliance."
    }
}