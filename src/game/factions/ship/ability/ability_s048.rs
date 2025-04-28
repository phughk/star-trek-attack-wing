use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS048{}
impl ShipAbility for AbilityS048 {
    fn get_description(&self) -> &'static str {
        "When you initiate an attack against a cloaked ship, roll +1 attack die."
    }
}