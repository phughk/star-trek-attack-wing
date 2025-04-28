use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS315 {}

impl ShipAbility for AbilityS315 {
    fn get_description(&self) -> &'static str {
        "<b>COMBAT PHASE:</b> Disable any number of Active Shields on this ship and target an opposing ship within Range 1-3.\n\nDisable the same number of Active Shields -1 on the target ship."
    }
}