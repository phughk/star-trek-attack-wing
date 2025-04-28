use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS133{}
impl ShipAbility for AbilityS133 {
    fn get_description(&self) -> &'static str {
        "You can perform a Red Maneuver while there is an Auxiliary Power Token beside your ship."
    }
}