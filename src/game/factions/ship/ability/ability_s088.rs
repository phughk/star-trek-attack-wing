use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS088{}
impl ShipAbility for AbilityS088 {
    fn get_description(&self) -> &'static str {
        "When attacking, during the Modify Attack Dice step, you may spend 3 Drone Tokens to choose any number of your attack dice and re-roll them (even if they have already been re-rolled)."
    }
}