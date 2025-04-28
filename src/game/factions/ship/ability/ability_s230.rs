use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS230{}
impl ShipAbility for AbilityS230 {
    fn get_description(&self) -> &'static str {
        "When attacking if you do not hit the defending ship, place an [evade] Token beside your ship."
    }
}