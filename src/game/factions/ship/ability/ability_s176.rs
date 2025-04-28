use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS176{}
impl ShipAbility for AbilityS176 {
    fn get_description(&self) -> &'static str {
        "You may use the [cloak] Action even if you have no Active Shields. Whenever you choose the [cloak] Action, roll 1 attack die. On a [battlestations] result, your Hull sustains 1 Damage."
    }
}