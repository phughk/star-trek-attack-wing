use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS290{}

impl ShipAbility for AbilityS290 {
    fn get_description(&self) -> &'static str {
        "<b>ACTIVATION PHASE:</b> After this ship executes a Green Maneuver, place 2 [time] Tokens on this card.\nRemove a Disable Token from 1 Upgrade equipped to this ship or equip a Ferengi Upgrade that was discarded from this ship to this ship."
    }
}