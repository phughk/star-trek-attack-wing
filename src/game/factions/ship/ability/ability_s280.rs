use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS280{}
impl ShipAbility for AbilityS280 {
    fn get_description(&self) -> &'static str {
        "\n<b>COMBAT PHASE:</b> After this ship attacks:\n\nThis ship may perform a Green 2 [bank-left] or Green 2 [bank-right] Maneuver."
    }
}