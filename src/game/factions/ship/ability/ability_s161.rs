use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS161{}
impl ShipAbility for AbilityS161 {
    fn get_description(&self) -> &'static str {
        "Each time you defend, you may convert up to 2 of your [battlestations] results into [evade] results."
    }
}