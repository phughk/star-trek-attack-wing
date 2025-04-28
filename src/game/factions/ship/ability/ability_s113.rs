use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS113{}
impl ShipAbility for AbilityS113 {
    fn get_description(&self) -> &'static str {
        "After you move, you may discard one of your Upgrades to perform an additional Green or White Maneuver.  You cannot deploy a [borg] Upgrade with a cost greater than 5 to this ship."
    }
}