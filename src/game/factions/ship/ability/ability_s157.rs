use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS157{}
impl ShipAbility for AbilityS157 {
    fn get_description(&self) -> &'static str {
        "Each time you defend, if there is an [evade] Token beside your ship, you may re-roll all of your defense dice once. You must keep the outcome of the 2nd roll."
    }
}