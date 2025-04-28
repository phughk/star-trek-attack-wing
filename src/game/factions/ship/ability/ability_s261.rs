use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS261{}
impl ShipAbility for AbilityS261 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN THIS SHIP WOULD BE DESTROYED:</b> \n\nThe attacking ship must discard one [crew] Upgrade. If it can't, place an [aux] Token beside it."
    }
}