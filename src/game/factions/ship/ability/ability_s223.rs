use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS223{}
impl ShipAbility for AbilityS223 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> If your ship is not Cloaked, when attacking with your Primary Weapon at Range 1 this round, gain +1 attack die and roll -1 defence die."
    }
}