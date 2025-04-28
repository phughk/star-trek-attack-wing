use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS026{}
impl ShipAbility for AbilityS026 {
    fn get_description(&self) -> &'static str {
        "You may deploy [weapon] Upgrades with a cost of 4 SP or less to this ship."
    }
}