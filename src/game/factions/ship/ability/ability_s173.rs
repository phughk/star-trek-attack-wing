use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS173{}
impl ShipAbility for AbilityS173 {
    fn get_description(&self) -> &'static str {
        "Each time you defend while Cloaked, roll +1 defense die."
    }
}