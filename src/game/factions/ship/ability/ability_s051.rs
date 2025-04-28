use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS051{}
impl ShipAbility for AbilityS051 {
    fn get_description(&self) -> &'static str {
        "At the start of the game, during the Gather Forces Step, you may purchase 1 additional [crew] or [tech] Upgrade for this ship.  This Upgrade may exceed the ship's restrictions and cost -2 SP (min 0)."
    }
}