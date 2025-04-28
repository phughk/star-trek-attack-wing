use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS006{}
impl ShipAbility for AbilityS006 {
    fn get_description(&self) -> &'static str {
        "Each time you defend, if you take no damage from an attack, you may immediately roll 1 attack die. A [hit] or [crit] result damages the attacking ship."
    }
}