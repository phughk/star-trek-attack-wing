use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS111{}
impl ShipAbility for AbilityS111 {
    fn get_description(&self) -> &'static str {
        "You cannot deploy a [borg] Upgrade with a cost greater than 5 to this ship."
    }
}