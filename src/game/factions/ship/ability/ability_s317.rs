use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS317 {}
impl ShipAbility for AbilityS317 {
    fn get_description(&self) -> &'static str {
        "\n<b>ACTION:</b>\n\nRepair up to 3 Shields on this ship."
    }
}