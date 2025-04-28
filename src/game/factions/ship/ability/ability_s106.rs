use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS106{}
impl ShipAbility for AbilityS106 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Spend 1 Drone Token to repair 1 damage to your Hull or Shields."
    }
}