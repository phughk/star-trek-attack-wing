use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS035{

}

impl ShipAbility for AbilityS035 {
    fn get_description(&self) -> &'static str {
        "When defending, you may re-roll 1 of your [battlestations] results. If your ship is cloaked, you may re-roll all of your [battlestations] results."
    }
}