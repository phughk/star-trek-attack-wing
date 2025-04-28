use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS061{}
impl ShipAbility for AbilityS061 {
    fn get_description(&self) -> &'static str {
        "During the End Phase of each round, repair 1 damage to your Hull or Shields (your choice).  This ship may only be assigned Gareb or a Romulan Drone Pilot as its Captain."
    }
}