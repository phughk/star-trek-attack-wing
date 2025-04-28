use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS292{}

impl ShipAbility for AbilityS292 {
    fn get_description(&self) -> &'static str {
        "\n<b>END PHASE:</b>\n\nAdd 1 Drone Token to the Captain equipped to this ship."
    }
}