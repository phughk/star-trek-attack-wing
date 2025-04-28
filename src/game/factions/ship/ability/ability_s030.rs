use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS030{}
impl ShipAbility for AbilityS030 {
    fn get_description(&self) -> &'static str {
        "When defending, if your ship is not cloaked and there is a friendly ship within Range 1-2 of your ship, you may roll +1 defense die. If you do so, place an Auxiliary Power Token beside your ship."
    }
}