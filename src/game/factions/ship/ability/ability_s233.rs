use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS233{}
impl ShipAbility for AbilityS233 {
    fn get_description(&self) -> &'static str {
        "When firing a Photon Torpedo Upgrade, place 1 less Time Token on that Upgrade."
    }
}