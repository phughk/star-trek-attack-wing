use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS129{}
impl ShipAbility for AbilityS129 {
    fn get_description(&self) -> &'static str {
        "While you are Cloaked, after you perform a Green Maneuver, you may perform a [sensor-echo] Action as a free Action."
    }
}