use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS254 {}
impl ShipAbility for AbilityS254 {
    fn get_description(&self) -> &'static str {
        "During the Activation Phase, if your ship is Cloaked and performs the [sensor-echo] Action, you may flip your [cloak] Token over to its red side and immediately perform an additional [sensor-echo] Action as a free Action."
    }
}