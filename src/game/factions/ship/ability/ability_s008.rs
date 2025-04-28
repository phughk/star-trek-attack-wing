use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS008{}
impl ShipAbility for AbilityS008 {
    fn get_description(&self) -> &'static str {
        "After you perform a Green Maneuver, you may immediately perform a [scan] Action as a free Action."
    }
}