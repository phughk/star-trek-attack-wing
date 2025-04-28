use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS216{}
impl ShipAbility for AbilityS216 {
    fn get_description(&self) -> &'static str {
        "When defending if you are hit and you don't already have a \"Warp Core Breach\" critical damage card assigned to your ship, you may chose to ignore up to 3 damage being inflicted to your ship and suffer a \"Warp Core Breach\" critical damage instead."
    }
}