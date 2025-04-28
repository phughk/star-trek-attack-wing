use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS044{}

impl ShipAbility for AbilityS044 {
    fn get_description(&self) -> &'static str {
        "After you move, if your ship is within Range 1 of a friendly ship you may perform a [battlestations] Action as a free Action."
    }
}