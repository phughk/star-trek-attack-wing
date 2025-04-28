use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS139{}
impl ShipAbility for AbilityS139 {
    fn get_description(&self) -> &'static str {
        "The Borg Ablative Hull Armor [borg] Upgrade costs -3 SP to equip to this ship."
    }
}