use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS011{}
impl ShipAbility for AbilityS011 {
    fn get_description(&self) -> &'static str {
        "When defending, if you have a [scan] Token beside your ship, roll +1 defense die."
    }
}