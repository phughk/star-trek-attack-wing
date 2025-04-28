use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS240{}
impl ShipAbility for AbilityS240 {
    fn get_description(&self) -> &'static str {
        "If this ship is a support ship, you may equip 1 Upgrade (of your choice) at -2 SP to this ship and you may use a 1 [straight], 2 [straight], or 3 [straight] Maneuver Template to place it in the play area when it becomes Active."
    }
}