use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS099{}
impl ShipAbility for AbilityS099 {
    fn get_description(&self) -> &'static str {
        "If there is a [scan] Token beside your ship during the Modify Defense Dice step of the Combat Phase, roll +1 defense die. \nYou cannot deploy a [borg] Upgrade with a cost greater than 5 to this ship."
    }
}