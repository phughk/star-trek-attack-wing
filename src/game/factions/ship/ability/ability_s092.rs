use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS092{}
impl ShipAbility for AbilityS092 {
    fn get_description(&self) -> &'static str {
        "During the Roll Attack Dice step of the Combat Phase, your ship, or 1 friendly ship within Range 1-2 of your ship, may spend a Scan Token from beside this ship to gain +1 attack die."
    }
}