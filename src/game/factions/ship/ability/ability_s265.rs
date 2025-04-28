use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS265{}
impl ShipAbility for AbilityS265 {
    fn get_description(&self) -> &'static str {
        "You cannot equip a Captain or Admiral to this ship.\n\nWhile this ship has no Damage cards, treat its Captain Skill as 12."
    }
}