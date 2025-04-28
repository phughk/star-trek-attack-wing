use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS058{}
impl ShipAbility for AbilityS058 {
    fn get_description(&self) -> &'static str {
        "When attacking with your Primary Weapon while cloaked, gain +1 attack die for every other Romulan Ship in your fleet (max +4)."
    }
}