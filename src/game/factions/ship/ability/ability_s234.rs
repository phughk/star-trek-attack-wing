use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS234{}
impl ShipAbility for AbilityS234 {
    fn get_description(&self) -> &'static str {
        "If all of your Shields have been destroyed, when attacking with your Primary Weapon, you may gain +2 attack dice. If you do so, place an Auxiliary Power Token beside your ship. You cannot roll any defence dice during a round in which you use this ability."
    }
}