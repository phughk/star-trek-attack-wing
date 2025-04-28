use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS253 {}

impl ShipAbility for AbilityS253 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Roll Defence Dice step, if your ship is not Cloaked, you may roll +2 defence dice. If you do so place an Auxiliary Power Token beside your ship."
    }
}