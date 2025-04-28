use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS235{}
impl ShipAbility for AbilityS235 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Compare Results step, you may discard 1 of your non-disabled Upgrades to cancel 1 [hit] or [crit] result."
    }
}