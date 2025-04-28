use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS010{}
impl ShipAbility for AbilityS010 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> If you are within Range 1-3 of at least 1 enemy ship, roll 3 defense dice. For every [evade] result, place 1 [evade] Token beside your ship. You cannot perform any free Actions this round."
    }
}