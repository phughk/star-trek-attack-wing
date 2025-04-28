use crate::game::factions::ship::ability::ability_s245::AbilityS245;
use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS244{}
impl ShipAbility for AbilityS244 {
    fn get_description(&self) -> &'static str {
        "When defending, if there is an Auxiliary Power Token beside your ship, during the Roll Defense Dice step, roll +1 defense die."
    }
}