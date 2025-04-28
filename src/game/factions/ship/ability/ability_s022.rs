use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS022{}
impl ShipAbility for AbilityS022 {
    fn get_description(&self) -> &'static str {
        "When attacking a ship at Range 3, if there is a [scan] Token beside your ship, the defending ship rolls -2 defense dice instead of -1."
    }
}