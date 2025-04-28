use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS213{}
impl ShipAbility for AbilityS213 {
    fn get_description(&self) -> &'static str {
        "You may perform an [evade] or a [scan] Action while there is an Auxiliary Power Token beside your ship."
    }
}