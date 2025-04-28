use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS337{}
impl ShipAbility for AbilityS337 {
    fn get_description(&self) -> &'static str {
        "<b>FREE ACTION:</b>\nPlace an [aux] Token beside this ship and target a friendly ship within Range 1-2.\n\nPlace a [scan] Token and an [evade] Token beside this ship and target ship."
    }
}