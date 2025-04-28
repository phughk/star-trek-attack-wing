use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS347 {}

impl ShipAbility for AbilityS347 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING A SHIP WITH A HULL OF 4 OR GREATER:</b> Steal a [scan] Token, a [battlestations] Token, or an [evade] Token from beside the defending ship and place it beside this ship.\n<b>WHEN ATTACKING A KLINGON SHIP OR A SHIP WITH A KLINGON CAPTAIN EQUIPPED TO IT:</b> Add 1 [hit]. "
    }
}