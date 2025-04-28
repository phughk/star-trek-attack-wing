use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS063{}
impl ShipAbility for AbilityS063 {
    fn get_description(&self) -> &'static str {
        "ATTACK SQUADRON TOKENS: 5\r\nWhen attacking a ship at Range 1, if that ship has at least 1 damage to its Hull, during the Modify Attack Dice step, add 1 [hit] result to your roll."
    }
}