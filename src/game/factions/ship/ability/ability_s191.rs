use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS191{}
impl ShipAbility for AbilityS191 {
    fn get_description(&self) -> &'static str {
        "Whenever a friendly ship within Range 1 of your ship receives damage you may transfer any amount of that damage to your own Shields, if possible."
    }
}