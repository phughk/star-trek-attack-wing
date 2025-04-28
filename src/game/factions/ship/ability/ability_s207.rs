use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS207{}
impl ShipAbility for AbilityS207 {
    fn get_description(&self) -> &'static str {
        "Instead of making a normal attack with your primary weapon, you may fire in any direction at Range 1-2 with 3 attack dice."
    }
}