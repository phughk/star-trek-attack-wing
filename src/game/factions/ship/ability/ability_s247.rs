use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS247 {}
impl ShipAbility for AbilityS247 {
    fn get_description(&self) -> &'static str {
        "During the End Phase, if your [cloak] Token is on its red side, you may flip it back to its green side. If you do so place an Auxiliary Power Token beside your ship."
    }
}