use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS150{}
impl ShipAbility for AbilityS150 {
    fn get_description(&self) -> &'static str {
        "Instead of making a normal attack with your Primary Weapon, you may fire in any direction at Range 1-2 with 4 attack dice. If you do so, place an Auxiliary Power Token beside your ship."
    }
}