use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS266{}

impl ShipAbility for AbilityS266 {
    fn get_description(&self) -> &'static str {
        "This ship may only be assigned Gareb or a Romulan Drone Pilot as its Captain."
    }
}