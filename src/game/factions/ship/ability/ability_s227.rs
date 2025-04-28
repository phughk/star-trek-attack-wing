use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS227 {}
impl ShipAbility for AbilityS227 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Roll Defence Dice step, you may roll 1 defence die for each [hit] or [crit] result rolled by the attacking ship. If you do so, Place an Auxiliary Power Token beside your ship."
    }
}