use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS148{}
impl ShipAbility for AbilityS148 {
    fn get_description(&self) -> &'static str {
        "When attacking a ship with a Scan token beside it with your Primary Weapon, roll an additional +2 attack dice."
    }
}