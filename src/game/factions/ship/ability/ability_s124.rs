use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS124{}
impl ShipAbility for AbilityS124 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Disable up to 2 of your Active Shields.  For each Shield you disabled with this Action, gain +1 attack die for all of your attacks with your Primary Weapon this round."
    }
}