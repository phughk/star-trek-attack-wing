use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS094{}
impl ShipAbility for AbilityS094 {
    fn get_description(&self) -> &'static str {
        "When you perform a [cloak] Action, you may immediately perform a [regenerate] Action as a free Action. \nIf you do so, you cannot attack that round."
    }
}