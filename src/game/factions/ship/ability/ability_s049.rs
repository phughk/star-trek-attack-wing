use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS049{}
impl ShipAbility for AbilityS049 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Roll 3 defense dice.  For every [evade] result, place 1 [evade] Token beside your ship."
    }
}