use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS311 {}

impl ShipAbility for AbilityS311 {
    fn get_description(&self) -> &'static str {
        "<b>END PHASE:</b> Flip 1 Specialization card equipped to this ship and target one face up damage card beside this card.\n\nFlip the target damage card face down."
    }
}