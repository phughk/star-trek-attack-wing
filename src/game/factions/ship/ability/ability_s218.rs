use crate::game::factions::ship::ability::ShipAbility;
pub struct AbilityS218{}
impl ShipAbility for AbilityS218 {
    fn get_description(&self) -> &'static str {
        "When Defending during the Compare Results step, you may place an Auxiliary Power Token beside your ship to cancel 1 [hit] result."
    }
}