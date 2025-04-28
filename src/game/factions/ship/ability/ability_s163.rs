use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS163{}
impl ShipAbility for AbilityS163 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Disable 1 of your Active Shields. During the End Phase this round, repair all of your damaged Shields."
    }
}