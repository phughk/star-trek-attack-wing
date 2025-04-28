use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS178{}
impl ShipAbility for AbilityS178 {
    fn get_description(&self) -> &'static str {
        "Any ship attacking you at Range 3 rolls 1 less attack die."
    }
}