use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS075{}
impl ShipAbility for AbilityS075 {
    fn get_description(&self) -> &'static str {
        "Instead of making a normal attack with your Primary Weapon, you may fire in any direction at Range 1-3 with 4 attack dice."
    }
}