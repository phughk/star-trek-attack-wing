use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS193{}
impl ShipAbility for AbilityS193 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Target a ship within your firing arc at Range 1-2 and roll 3 attack dice. For each [hit] result disable 1 of that ship's Shields. If possible, you may play Klingon Boarding Party as a free action this turn. "
    }
}