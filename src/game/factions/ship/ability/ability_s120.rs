use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS120{}
impl ShipAbility for AbilityS120 {
    fn get_description(&self) -> &'static str {
        "Each time you defend, if you are within Range 1-2 of at least one friendly Jem'Hadar Attack Ship, roll 1 extra defense die."
    }
}