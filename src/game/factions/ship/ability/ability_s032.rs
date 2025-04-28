use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS032{}

impl ShipAbility for AbilityS032 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Compare Results step, you may cancel 1 [hit] result. If you do so, place an Auxiliary Power Token beside your ship."
    }
}