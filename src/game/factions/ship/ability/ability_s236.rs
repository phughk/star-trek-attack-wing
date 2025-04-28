use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS236 {}

impl ShipAbility for AbilityS236 {
    fn get_description(&self) -> &'static str {
        "When attacking with your Primary Weapon, during the Modify Attack Dice step, you may disable any number of your Active Shields to re-roll a number of your attack dice equal to the number of Shields you disabled."
    }
}