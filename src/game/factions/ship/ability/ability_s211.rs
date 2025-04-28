use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS211{}
impl ShipAbility for AbilityS211 {
    fn get_description(&self) -> &'static str {
        "During the Activation Phase, you may perform a second Action from your Action Bar as a free Action. If you do so, place an Auxiliary Power Token beside your ship."
    }
}