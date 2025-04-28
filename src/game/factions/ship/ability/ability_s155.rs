use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS155{}
impl ShipAbility for AbilityS155 {
    fn get_description(&self) -> &'static str {
        "If you initiate an attack at Range 1, you may re-roll 1 of your attack dice. \n\n(Initial Captain skill 6)."
    }
}