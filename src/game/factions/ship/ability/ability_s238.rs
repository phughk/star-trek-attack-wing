use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS238{}
impl ShipAbility for AbilityS238 {
    fn get_description(&self) -> &'static str {
        "When attacking with your Primary Weapon, if your ship is not Cloaked, during the Declare Target step, you may perform a 1 [straight] Maneuver before choosing an enemy ship to attack."
    }
}