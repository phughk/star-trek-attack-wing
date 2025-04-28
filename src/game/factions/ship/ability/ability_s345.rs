use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS345{}

impl ShipAbility for AbilityS345 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING:</b> If this ship is Cloaked, disable a Klingon [crew] Upgrade equipped to this ship\n\nDo not flip this ship's [cloak] Token to its red side and place a [battlestations] Token beside this ship."
    }
}