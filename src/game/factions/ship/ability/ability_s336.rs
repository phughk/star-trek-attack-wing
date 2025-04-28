use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS336 {}

impl ShipAbility for AbilityS336 {
    fn get_description(&self) -> &'static str {
        "You may equip a Klingon [tech] Upgrade with a printed cost of 4 SP or less to this ship for free, even if it exceeds this ship's retrictions."
    }
}