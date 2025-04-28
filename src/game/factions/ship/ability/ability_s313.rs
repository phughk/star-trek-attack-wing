use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS313 {
    
}

impl ShipAbility for AbilityS313 {
    fn get_description(&self) -> &'static str {
        "You may equip the Inertial Compensators Upgrade to this ship for free and without requiring an Upgrade Slot.\n<b>WHEN ATTACKING:</b> If the defending ship is hit: Place an [aux] token beside the defending ship."
    }
}