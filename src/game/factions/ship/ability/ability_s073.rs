use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS073{}
impl ShipAbility for AbilityS073 {
    fn get_description(&self) -> &'static str {
        "Each time you defend, during the Roll Defense Dice step, you may roll up to 2 additional defense dice.  If you do, disable 1 of your Active Shields for each additional die you roll."
    }
}