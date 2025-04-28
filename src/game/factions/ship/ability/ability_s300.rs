use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS300{}

impl ShipAbility for AbilityS300 {
    fn get_description(&self) -> &'static str {
        "<b>MODIFY ATTACK DICE STEP:\n</b> Disable 1 Shield on this ship.\n\nCancel 1 [hit] or [crit]."
    }
}