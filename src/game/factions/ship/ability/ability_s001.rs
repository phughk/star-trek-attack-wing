use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS001{}
impl ShipAbility for AbilityS001 {
    fn get_description(&self) -> &'static str {
        "If you reveal a 4 [forward] maneuver, immediately before you move, you may change it to a 4 Bank maneuver. Treat this as a Red Maneuver."
    }
}