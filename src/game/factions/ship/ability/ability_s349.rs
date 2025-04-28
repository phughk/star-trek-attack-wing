use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS349 {}

impl ShipAbility for AbilityS349 {
    fn get_description(&self) -> &'static str {
        "<b>FREE ACTION:</b> \n\nPerform a [sensor-echo] Action even if this ship isn't Cloaked.  If this ship is Cloaked, you may use a 3 [straight] Manuever Template for this Action."
    }
}