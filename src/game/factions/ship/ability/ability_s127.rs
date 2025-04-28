use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS127{}
impl ShipAbility for AbilityS127 {
    fn get_description(&self) -> &'static str {
        "When attacking with your Primary Weapon, during the Roll Attack Dice step of the Combat Phase you may disable one of your [crew] Upgrades to add +1 attack die to your attack."
    }
}