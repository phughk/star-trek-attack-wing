use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS284 {}

impl ShipAbility for AbilityS284 {
    fn get_description(&self) -> &'static str {
        "\nThis ship rolls +1 attack die if there is another Jem'Hadar Attack Ship within Range 1."
    }
}