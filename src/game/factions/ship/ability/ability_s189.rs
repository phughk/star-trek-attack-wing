use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS189 {}

impl ShipAbility for AbilityS189 {
    fn get_description(&self) -> &'static str {
        "Although you must be Target Locked on a ship to fire a Torpedo at it, you do not need to discard your Target Lock when doing so."
    }
}