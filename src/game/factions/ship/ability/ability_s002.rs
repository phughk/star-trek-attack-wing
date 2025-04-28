use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS002{}
impl ShipAbility for AbilityS002 {
    fn get_description(&self) -> &'static str {
        "When attacking a ship with your Primary Weapon that has a larger Hull value and/or Primary Weapon value than your ship, gain +1 attack die."
    }
}