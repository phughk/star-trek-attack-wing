use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS019{}

impl ShipAbility for AbilityS019 {
    fn get_description(&self) -> &'static str {
        "When attacking a ship at Range 3, during the Roll Attack Dice step, gain +1 attack die for every friendly ship within Range 1 of your ship (max +2)."
    }
}