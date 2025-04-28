use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS038{}
impl ShipAbility for AbilityS038 {
    fn get_description(&self) -> &'static str {
        "When a ship that started the Activation Phase in your forward firing arc moves, if you have not yet moved, you may immediately change your maneuver. You may only use this ability once per round."
    }
}