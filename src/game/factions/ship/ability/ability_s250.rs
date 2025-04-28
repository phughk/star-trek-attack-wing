use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS250{}
impl ShipAbility for AbilityS250 {
    fn get_description(&self) -> &'static str {
        "When attacking, during the Modify Attack Dice step, you may spend your [battlestations] Token to convert one of your [hit] results into a [crit] result."
    }
}