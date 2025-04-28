use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS202{}
impl ShipAbility for AbilityS202 {
    fn get_description(&self) -> &'static str {
        "You gain +1 attack die when firing at Range 1."
    }
}