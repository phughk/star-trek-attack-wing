use crate::game::factions::ship::ability::ability_s171::AbilityS171;
use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS169{}
impl ShipAbility for AbilityS169 {
    fn get_description(&self) -> &'static str {
        "After you move, if no enemy ships are within Range 1 of your ship, you may perform a [scan] Action as a free Action."
    }
}