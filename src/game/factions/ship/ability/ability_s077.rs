use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS077{}
impl ShipAbility for AbilityS077 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Target a ship at Range 1 and force that ship to discard 1 of its Active Shield Tokens.  Place an Auxiliary Power Token beside your ship."
    }
}