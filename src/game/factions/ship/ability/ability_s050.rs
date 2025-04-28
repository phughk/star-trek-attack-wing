use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS050{}
impl ShipAbility for AbilityS050 {
    fn get_description(&self) -> &'static str {
        "During the Roll Attack Dice step, if there is an Auxiliary Power Token beside your ship, gain +2 attack dice when attacking with your Primary Weapon."
    }
}