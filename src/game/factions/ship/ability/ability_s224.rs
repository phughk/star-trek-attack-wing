use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS224{}
impl ShipAbility for AbilityS224 {
    fn get_description(&self) -> &'static str {
        "Attack Squadron Tokens: 4"
    }
}