use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS168{}
impl ShipAbility for AbilityS168 {
    fn get_description(&self) -> &'static str {
        "Usable in Star Trek Alliance.\n(Captain Skill 2+)."
    }
}