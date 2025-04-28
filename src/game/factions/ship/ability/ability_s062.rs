use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS062{}
impl ShipAbility for AbilityS062 {
    fn get_description(&self) -> &'static str {
        "ATTACK SQUADRON TOKENS: 4"
    }
}