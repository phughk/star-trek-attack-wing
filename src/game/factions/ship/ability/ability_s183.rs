use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS183{}
impl ShipAbility for AbilityS183 {
    fn get_description(&self) -> &'static str {
        "If you initiate an attack while cloaked, you may choose any number of your attack dice and re-roll them once."
    }
}