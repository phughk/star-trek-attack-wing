use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS259 {}
impl ShipAbility for AbilityS259 {
    fn get_description(&self) -> &'static str {
        "You may deploy [weapon] Upgrades with a cost of 4 SP or less to this ship.\n\nYou may place an Auxiliary Power Token next to your ship to perform an [battlestations] Action."
    }
}