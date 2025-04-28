use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS020{}
impl ShipAbility for AbilityS020 {
    fn get_description(&self) -> &'static str {
        "When attacking, during the Modify Attack Dice step, you may convert 1 of your [hit] results into 1 [crit] result. If you do so, place an Auxiliary Power Token beside your ship."
    }
}