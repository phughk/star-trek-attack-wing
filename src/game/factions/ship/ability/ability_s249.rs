use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS249{}
impl ShipAbility for AbilityS249 {
    fn get_description(&self) -> &'static str {
        "When defending, During the Modify Defense dice step, you may re-roll one of your defense dice. If you do so, place an Auxiliary Power Token beside your ship."
    }
}