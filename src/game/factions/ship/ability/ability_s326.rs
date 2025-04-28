use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS326 {}

impl ShipAbility for AbilityS326 {
    fn get_description(&self) -> &'static str {
        "<b>FREE ACTION:</b> If this ship performed a Green Maneuver this game round:\n\n Perform a &[target-lock] Action as a Free Action"
    }
}
