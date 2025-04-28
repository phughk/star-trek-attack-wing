use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS072{}
impl ShipAbility for AbilityS072 {
    fn get_description(&self) -> &'static str {
        "ATTACK SQUADRON TOKENS: 5\r\nDuring the Roll Attack Dice step, you may disable one of your [squadron] Upgrade to gain +1 additional attack die for that attack."
    }
}