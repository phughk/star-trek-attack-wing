use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS126{}
impl ShipAbility for AbilityS126 {
    fn get_description(&self) -> &'static str {
        "When attacking with Photon Torpedoes, you do not need to disable the Photon Torpedoes."
    }
}