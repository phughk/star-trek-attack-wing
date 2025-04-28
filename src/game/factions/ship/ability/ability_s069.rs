use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS069{}
impl ShipAbility for AbilityS069 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Declare Target step, you may spend 1 Drone Token to set 1 of your defense dice on any side that you want.  This die cannot be rolled or re-rolled during that attack."
    }
}