use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS248{}
impl ShipAbility for AbilityS248 {
    fn get_description(&self) -> &'static str {
        "During the Combat Phase, you may replace 1 [battlestations], [scan] or blue &[target-lock] Token that is beside your ship with an [evade] Token."
    }
}