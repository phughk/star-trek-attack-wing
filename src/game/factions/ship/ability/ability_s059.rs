use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS059{}
impl ShipAbility for AbilityS059 {
    fn get_description(&self) -> &'static str {
        "All friendly Kazon ships with a Hull value of 4 or less within Range 1-2 of this ship gain +1 defense die."
    }
}