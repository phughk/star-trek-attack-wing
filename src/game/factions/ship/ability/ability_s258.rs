use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS258 {}

impl ShipAbility for AbilityS258 {
    fn get_description(&self) -> &'static str {
        "When attacking, if your ship is not in the target ship's forward firing arc, during the Modify Attack Dice step, you may re-roll all of your blank results."
    }
}