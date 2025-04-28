use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS043{}
impl ShipAbility for AbilityS043 {
    fn get_description(&self) -> &'static str {
        "While you are cloaked, if you initiate an attack at Range 1 with your Primary Weapon, gain +1 attack die for that attack."
    }
}