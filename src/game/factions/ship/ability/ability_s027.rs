use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS027{}
impl ShipAbility for AbilityS027 {
    fn get_description(&self) -> &'static str {
        "You may deploy [weapon] Upgrades with a cost of 4 SP or less to this ship.\n\nYou may fire your Primary Weapon from your rear firing arc at -1 attack die."
    }
}