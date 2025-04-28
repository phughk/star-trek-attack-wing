use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS289{}
impl ShipAbility for AbilityS289 {
    fn get_description(&self) -> &'static str {
        "<b>END PHASE:</b> If this ship performed a Green Maneuver this game round.\n\nRepair 1 Hull or Shield."
    }
}