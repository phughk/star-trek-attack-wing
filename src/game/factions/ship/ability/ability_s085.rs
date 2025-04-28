use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS085{}
impl ShipAbility for AbilityS085 {
    fn get_description(&self) -> &'static str {
        "If you attack with Torpedoes while Cloaked, you do not flip your [cloak] Token over to its red side."
    }
}