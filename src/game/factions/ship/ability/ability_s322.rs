use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS322 {}

impl ShipAbility for AbilityS322 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING:</b> If the defending ship performed a maneuver with a speed of 3 or greater this game round:\n\nAdd 1 [crit]."
    }
}