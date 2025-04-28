use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS256 {}

impl ShipAbility for AbilityS256 {
    fn get_description(&self) -> &'static str {
        "During the Roll Attack Dice or Roll Defence Dice step of the Combat Phase, you may switch and [evade], [scan], or &[target-lock] Token that is beside your ship for a [battlestations] Token."
    }
}