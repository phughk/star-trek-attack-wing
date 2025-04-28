use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS100{}
impl ShipAbility for AbilityS100 {
    fn get_description(&self) -> &'static str {
        "If you performed a Green Maneuver this round, during the Roll Attack Dice step of the Combat Phase, roll +1 attack die."
    }
}