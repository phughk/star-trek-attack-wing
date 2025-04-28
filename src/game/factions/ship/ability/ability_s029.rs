use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS029{}
impl ShipAbility for AbilityS029 {
    fn get_description(&self) -> &'static str {
        "Each time you perform an Action or use an ability on any of your [tech] Upgrades, place 1 Mission Token on this card. During the Roll Attack Dice step, you may spend up to 2 of these tokens to gain +1 attack die for that attack for each token spent."
    }
}