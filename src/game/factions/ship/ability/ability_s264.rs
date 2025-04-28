use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS264{}
impl ShipAbility for AbilityS264 {
    fn get_description(&self) -> &'static str {
        "You cannot equip a Captain or Admiral to this ship."
    }
}