use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS198{}
impl ShipAbility for AbilityS198 {
    fn get_description(&self) -> &'static str {
        "If you perform a Green Maneuver, add +1 attack die for the remainder of this round."
    }
}