use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS287{}
impl ShipAbility for AbilityS287 {
    fn get_description(&self) -> &'static str {
        "\nJem'Hadar Attack Ships within Range 1 roll +1 defense dice"
    }
}