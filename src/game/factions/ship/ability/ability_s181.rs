use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS181{}
impl ShipAbility for AbilityS181 {
    fn get_description(&self) -> &'static str {
        "Each time you attack, you may re-roll 1 of your attack dice for every damage card assigned to your ship."
    }
}