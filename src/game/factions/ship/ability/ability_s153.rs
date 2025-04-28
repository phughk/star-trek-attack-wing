use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS153{}
impl ShipAbility for AbilityS153 {
    fn get_description(&self) -> &'static str {
        "Each time you attack with your Primary Weapon, you may divide your attack between 2 different ships; you may divide your attack dice however you like, but you must roll at least 1 die against each ship."
    }
}