use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS209 {}
impl ShipAbility for AbilityS209 {
    fn get_description(&self) -> &'static str {
        "During the Activation Phase you may perform a [scan] Action as a free Action. If you do so place an Auxiliary Power Token beside your ship."
    }
}