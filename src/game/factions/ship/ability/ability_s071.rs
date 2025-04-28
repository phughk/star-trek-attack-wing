use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS071{}
impl ShipAbility for AbilityS071 {
    fn get_description(&self) -> &'static str {
        "ATTACK SQUADRON TOKENS: 4"
    }
}