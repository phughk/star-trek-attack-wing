use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS056{}
impl ShipAbility for AbilityS056 {
    fn get_description(&self) -> &'static str {
        "When attacking with your Primary Weapon, if you inflict a Critical Damage to an enemy ship's hull, you may search the Damage Deck for a \"Structural Damage\" or a \"Weapons Malfunction\" card instead of drawing a random Damage Card.  Reshuffle the deck when you are done."
    }
}