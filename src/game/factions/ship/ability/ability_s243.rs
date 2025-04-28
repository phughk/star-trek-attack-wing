use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS243 {
    
}

impl ShipAbility for AbilityS243 {
    fn get_description(&self) -> &'static str {
        "When attacking during the Roll Attack Dice step, if there is a [scan] Token beside your ship gain +1 attack die (+2 if the defending ship is Cloaked)."
    }
}