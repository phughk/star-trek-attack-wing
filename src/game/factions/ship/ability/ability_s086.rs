use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS086{}
impl ShipAbility for AbilityS086 {
    fn get_description(&self) -> &'static str {
        "When defending, during the Compare Results step, you may discard up to 3 of your [borg] Upgrades.  Cancel 1 [hit] or [crit] result for each Upgrade you discard with this card."
    }
}