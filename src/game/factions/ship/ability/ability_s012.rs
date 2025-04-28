use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS012{}

impl ShipAbility for AbilityS012 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Perform a 2nd maneuver on your maneuver dial with a speed of 1 or 2. Place an Auxiliary Power Token beside your ship. You cannot attack this round."
    }
}