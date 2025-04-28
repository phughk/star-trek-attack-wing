use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS037{}
impl ShipAbility for AbilityS037 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Roll Attack Dice step, you may discard a Token from beside your ship ( [evade], [scan], or [battlestations] ) to force the attacking ship to roll 1 less attack die."
    }
}