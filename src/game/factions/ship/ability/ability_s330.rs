use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS330{}
impl ShipAbility for AbilityS330 {
    fn get_description(&self) -> &'static str {
        "<b>COMBAT PHASE:</b> Spend a [scan] Token from beside this ship.\nThe next attack this ship makes with its Primary Weapon this game round ignores the defending ship's Shields."
    }
}