use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS187{}
impl ShipAbility for AbilityS187 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Disable 1 of your Active Shields to gain +1 attack die this round. You may re-roll all of your blank results once."
    }
}