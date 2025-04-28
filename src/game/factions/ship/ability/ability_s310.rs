use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS310 {}

impl ShipAbility for AbilityS310 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> If this ship has one or more destroyed Shields, flip 1 Specialization card equipped to this ship.\n\n Repair 1 Shield on this ship."
    }
}