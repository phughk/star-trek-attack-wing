use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS034{}
impl ShipAbility for AbilityS034 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Compare Results step, you may disable 2 of your Active Shields to cancel 1 of the attacking ship's [hit] results."
    }
}