use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS220{}
impl ShipAbility for AbilityS220 {
    fn get_description(&self) -> &'static str {
        "When defending during the Roll Defense Dice step, you may disable one of your Active Shields to gain +2 defence dice for that attack."
    }
}