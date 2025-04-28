use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS221{}
impl ShipAbility for AbilityS221 {
    fn get_description(&self) -> &'static str {
        "You cannot assign a Captain or an Admiral to this ship. \nWhen defending, during the Compare Results step, you may discard up to 2 of your Upgrades to cancel an equal number of attacking ship's dice."
    }
}