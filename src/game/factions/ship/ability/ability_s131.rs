use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS131{}
impl ShipAbility for AbilityS131 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Disable up to 2 of your [crew] Upgrades and add +1 attack die to each of your attacks this round for each [crew] Upgrade you disabled with this Action."
    }
}