use crate::game::factions::ship::ability::ability_s160::AbilityS160;
use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS158{}
impl ShipAbility for AbilityS158 {
    fn get_description(&self) -> &'static str {
        "Each round, one other friendly Jem'Hadar ship within Range 1-2 of your ship may perform an Action on their Action Bar as a free Action."
    }
}