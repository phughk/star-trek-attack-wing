use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS093{}
impl ShipAbility for AbilityS093 {
    fn get_description(&self) -> &'static str {
        "During the Roll Attack Dice step of the Combat Phase, you may roll +1 attack die.  If you do so, suffer 1 damage to your Hull."
    }
}