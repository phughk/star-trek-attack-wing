use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS004{}
impl ShipAbility for AbilityS004 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Place a [scan] Token and an Auxiliary Power Token beside your ship."
    }
}