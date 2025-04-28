use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS039{}
impl ShipAbility for AbilityS039 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> If you performed a [forward] Maneuver this round, immediately perform an additional 1 [forward] or 2 [forward] Maneuver."
    }
}