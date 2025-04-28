use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS080{
}

impl ShipAbility for AbilityS080 {
    fn get_description(&self) -> &'static str {
        "<b>( ERRATA )</b>"
    }
}