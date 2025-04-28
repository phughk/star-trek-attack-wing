use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS215{}
impl ShipAbility for AbilityS215 {
    fn get_description(&self) -> &'static str {
        "When attacking with Photon Torpedoes during the Roll Attack Dice step, you may discard the Photon Torpedoes Upgrade to gain 1 additional attack die."
    }
}