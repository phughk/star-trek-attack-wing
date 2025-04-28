use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS070{}
impl ShipAbility for AbilityS070 {
    fn get_description(&self) -> &'static str {
        "During the Compare Results step, you may disable 1 of your Shields to cancel 1 [hit] or [crit] result. \n\nYou cannot deploy this card to the same fleet as \"Assimilated Vessel 64758.\""
    }
}