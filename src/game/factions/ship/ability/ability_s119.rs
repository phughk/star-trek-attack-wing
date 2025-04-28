use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS119{}
impl ShipAbility for AbilityS119 {
    fn get_description(&self) -> &'static str {
        "Whenever you initiate an attack, you may disable one of your non-disabled [tech] Upgrades to add +1 attack die."
    }
}