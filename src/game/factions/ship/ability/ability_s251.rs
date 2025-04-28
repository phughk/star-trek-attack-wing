use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS251{}
impl ShipAbility for AbilityS251 {
    fn get_description(&self) -> &'static str {
        "If your ship is within Range 1 of a friendly ship, each time it defends, it rolls +1 defense die. This ability may only be used for 1 friendly ship per round."
    }
}