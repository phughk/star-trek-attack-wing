use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS053{}
impl ShipAbility for AbilityS053 {
    fn get_description(&self) -> &'static str {
        "When attacking a ship that you already have target locked, during the Roll Attack Dice step, gain +1 attack die."
    }
}