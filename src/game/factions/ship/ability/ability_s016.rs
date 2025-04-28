use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS016{}
impl ShipAbility for AbilityS016 {
    fn get_description(&self) -> &'static str {
        "After you perform a maneuver with a speed of 5 or greater, you may immediately perform an [evade] Action as a free Action."
    }
}