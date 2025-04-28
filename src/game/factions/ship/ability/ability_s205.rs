use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS205{}
impl ShipAbility for AbilityS205 {
    fn get_description(&self) -> &'static str {
        "If you initiate an attack while cloaked, add +1 attack die."
    }
}