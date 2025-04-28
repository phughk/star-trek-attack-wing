use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS142{}
impl ShipAbility for AbilityS142 {
    fn get_description(&self) -> &'static str {
        "Whenever you initiate an attack against an enemy ship at Range 3, gain +1 attack die."
    }
}