use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS110{}
impl ShipAbility for AbilityS110{
    fn get_description(&self) -> &'static str {
        "During the Activation Phase, you may disable 1 of your Active Shields to remove 1 Auxiliary Power Token from beside your ship."
    }
}