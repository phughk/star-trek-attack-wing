use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS180{}
impl ShipAbility for AbilityS180 {
    fn get_description(&self) -> &'static str {
        "When defending, convert all of your opponent's [crit] results into [hit] results.\r\n      "
    }
}