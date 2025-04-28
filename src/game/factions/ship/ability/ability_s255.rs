use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS255{}
impl ShipAbility for AbilityS255 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Modify Defense Dice step, you may discard a blue &[target-lock] Token from beside your ship to re-roll all of your defense dice."
    }
}