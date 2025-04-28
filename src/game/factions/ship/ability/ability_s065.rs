use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS065{}
impl ShipAbility for AbilityS065 {
    fn get_description(&self) -> &'static str {
        "After you move, you may perform a [sensor-echo] Action with the 1 [forward] Maneuver as a free Action, even if you are not Cloaked."
    }
}