use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS185{}
impl ShipAbility for AbilityS185 {
    fn get_description(&self) -> &'static str {
        "When firing any torpedoes, do not disable the torpedoes."
    }
}