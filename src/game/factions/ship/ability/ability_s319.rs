use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS319 {}

impl ShipAbility for AbilityS319 {
    fn get_description(&self) -> &'static str {
        "<b>BEFORE THIS SHIP REVEALS ITS MANEUVER DIAL:</b> If this ship is Cloaked: \n\nPerform a white 1 [bank-left], 1 [straight], 1 [bank-right], OR perform a [sensor-echo] Action as a Free Action."
    }
}