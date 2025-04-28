use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS241{}
impl ShipAbility for AbilityS241 {
    fn get_description(&self) -> &'static str {
        "Your shields cannot be affected by Upgrades from an opponent's fleet."
    }
}