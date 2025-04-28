use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS341 {

}

impl ShipAbility for AbilityS341 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 2+)."
    }
}