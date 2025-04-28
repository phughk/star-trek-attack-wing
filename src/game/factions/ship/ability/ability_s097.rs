use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS097{}
impl ShipAbility for AbilityS097 {
    fn get_description(&self) -> &'static str {
        "During the Modify Defense Dice step of the Combat Phase, you may disable up to 3 of your Upgrades to add 1 [evade] result to your roll for each Upgrade you disabled with this card."
    }
}