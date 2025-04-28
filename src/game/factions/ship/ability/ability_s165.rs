use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS165{}
impl ShipAbility for AbilityS165 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Perform a [sensor-echo] Action even if this ship is not Cloaked. You may only use the 1 [forward] Maneuver Template for this Action."
    }
}