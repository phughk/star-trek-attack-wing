use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS098{}
impl ShipAbility for AbilityS098 {
    fn get_description(&self) -> &'static str {
        "Each time you defend, during the Modify Defense Dice step of the Combat Phase, you may add 1 [evade] result to your roll.  If you do so, place 1 Auxiliary Power Token beside your ship."
    }
}