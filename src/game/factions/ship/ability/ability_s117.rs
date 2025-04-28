use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS117{}
impl ShipAbility for AbilityS117 {
    fn get_description(&self) -> &'static str {
        "Each time you attack or defend, if there is a [scan] token beside your ship, your range combat bonuses are doubled."
    }
}