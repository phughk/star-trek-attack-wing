use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS108{}
impl ShipAbility for AbilityS108 {
    fn get_description(&self) -> &'static str {
        "During the Roll Attack Dice step of the Combat Phase, you may disable 1 of your Active Shields to gain +1 attack die."
    }
}