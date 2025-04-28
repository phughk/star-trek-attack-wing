use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS278 {}

impl ShipAbility for AbilityS278 {
    fn get_description(&self) -> &'static str {
        "<b>ACTIVATION PHASE:</b> Place 2 [time] Tokens on this card and target a friendly Romulan ship within Range 1.\n\nThe target ship may perform an Action as a Free Action."
    }
}