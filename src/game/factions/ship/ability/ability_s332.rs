use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS332 {}

impl ShipAbility for AbilityS332 {
    fn get_description(&self) -> &'static str {
        "If there are only Dominion and/or Romulan cards equipped to this ship, add the [cloak] and [sensor-echo] Actions to this ship's Action Bar.\n<b>WHEN A FRIENDLY SHIP WITHIN RANGE 1 PERFORMS THE CLOAK ACTION:</b> This ship may perform the [cloak] Action as a Free Action."
    }
}