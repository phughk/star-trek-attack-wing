use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS338{}

impl ShipAbility for AbilityS338 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING:</b> During the Modify Attack Dice Step, if there is a [scan] Token beside this ship.\n\nConvert up to 2 [battlestations] into 1 [hit] each."
    }
}