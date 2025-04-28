use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS079{}
impl ShipAbility for AbilityS079 {
    fn get_description(&self) -> &'static str {
        "Instead of making a normal attack with your Primary Weapon, you may fire at up to 4 different ships at Range 1-2 with 4 attack dice against each ship. If you do this, spend 1 Drone Token for each of these attacks after the first."
    }
}