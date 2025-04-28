use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS257 {}

impl ShipAbility for AbilityS257{
    fn get_description(&self) -> &'static str {
        "When attacking, during the Modify Attack Dice step, you may disable one of your Active Shields to re-roll up to 2 of your attack dice."
    }
}
