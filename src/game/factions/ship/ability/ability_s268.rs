use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS268 {}

impl ShipAbility for AbilityS268 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING:</b> If this ship is Cloaked.\n\nIf the defending ship is at Range 1, this ship rolls +2 attack dice. If the defending ship is at Range 2 this ship rolls +1 attack die."
    }
}