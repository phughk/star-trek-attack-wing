use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS267 {}
impl ShipAbility for AbilityS267 {
    fn get_description(&self) -> &'static str {
        "<b>PLANNING PHASE:</b> Place an [aux] Token beside this ship.\nRemove 1 [time] Token from each [tech] Upgrade equipped to this ship.\nThis ship may only be assigned Gareb or a Romulan Drone Pilot as its Captain."
    }
}