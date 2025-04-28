use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS260{}
impl ShipAbility for AbilityS260 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 2+).\n\nCard and model did not come with Star Trek Alliance."
    }
}