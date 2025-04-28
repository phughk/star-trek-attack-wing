use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS285{}
impl ShipAbility for AbilityS285 {
    fn get_description(&self) -> &'static str {
        "\nThis ship rolls +2 defense dice if there is another Jem'Hadar Attack Ship within Range 1."
    }
}