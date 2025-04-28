use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS203{}
impl ShipAbility for AbilityS203 {
    fn get_description(&self) -> &'static str {
        "Any ship attacking you at Range 1 rolls 1 less attack die."
    }
}