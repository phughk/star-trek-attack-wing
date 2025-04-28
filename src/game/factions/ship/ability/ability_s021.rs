use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS021{}
impl ShipAbility for AbilityS021 {
    fn get_description(&self) -> &'static str {
        "When attacking a ship that has damaged Shields, during the Modify Attack Dice step, you may re-roll a number of your Attack dice up to the number of the defending ship's damaged Shields."
    }
}