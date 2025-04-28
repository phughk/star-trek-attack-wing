use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS231{}
impl ShipAbility for AbilityS231 {
    fn get_description(&self) -> &'static str {
        "While there is a [scan] Token beside your ship, you cannot be targeted by any [talent] Upgrades and you roll +1 defense die against all attacks made with [weapon] Upgrades."
    }
}