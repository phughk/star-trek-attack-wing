use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS005{}
impl ShipAbility for AbilityS005 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 2+)."
    }
}