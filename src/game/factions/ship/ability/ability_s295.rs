use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS295{}

impl ShipAbility for AbilityS295 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN THIS SHIP PERFORMS A 3 [turn-left], 3 [bank-left], 3 [straight], 3 [bank-right], OR 3 [turn-right] MANEUVER:</b>\n\nThis ship may perform a White Maneuver. If it does, it rolls -1 attack die this game round."
    }
}