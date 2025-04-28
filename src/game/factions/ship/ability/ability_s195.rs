use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS195{}
impl ShipAbility for AbilityS195 {
    fn get_description(&self) -> &'static str {
        "Each time you defend against a ship at Range 1, roll 1 extra defense die."
    }
}