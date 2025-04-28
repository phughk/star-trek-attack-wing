use crate::game::factions::ship::ability::ability_s117::AbilityS117;
use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS115{}
impl ShipAbility for AbilityS115 {
    fn get_description(&self) -> &'static str {
        "You may equip the Enhanced Hull Plating [tech] Upgrade to your ship for free even if it exceeds your ship's restrictions."
    }
}