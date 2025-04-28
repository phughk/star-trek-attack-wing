use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS104{}
impl ShipAbility for AbilityS104 {
    fn get_description(&self) -> &'static str {
        "If you performed a Maneuver of 3 or higher this round, during the Roll Attack Dice step of the Combat Phase, you may add +1 attack die.  If you do so, place an Auxiliary Power Token beside your ship."
    }
}