use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS031{}
impl ShipAbility for AbilityS031 {
    fn get_description(&self) -> &'static str {
        "During the Deal Damage step, if you inflict 2 critical damage to an enemy ship's hull, you may immediately flip those damage cards face down and rotate the damaged ship's facing by 90 degrees (right or left)."
    }
}