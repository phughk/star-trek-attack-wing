use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS047{}
impl ShipAbility for AbilityS047 {
    fn get_description(&self) -> &'static str {
        "All of your [tech] Upgrades cost -1 SP."
    }
}