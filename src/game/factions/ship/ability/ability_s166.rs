use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS166{}
impl ShipAbility for AbilityS166 {
    fn get_description(&self) -> &'static str {
        "When you initiate an attack at range 3, you may choose any number of your attack dice and re-roll them once."
    }
}