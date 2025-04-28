use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS114{}
impl ShipAbility for AbilityS114 {
    fn get_description(&self) -> &'static str {
        "Whenever you attack an enemy ship at Range 3 with your Primary Weapon, if there is a [scan] Token beside your ship, you gain +1 attack die for that attack."
    }
}