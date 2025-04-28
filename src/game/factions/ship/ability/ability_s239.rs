use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS239{}
impl ShipAbility for AbilityS239 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Once this round, if you inflict a critical damage on an enemy ship's Hull, you may search the Damaage Deck for a \"Direct Hit\" damage card and place it beside the enemy's Ship Card. Shuffle the Damage Deck when you are done."
    }
}