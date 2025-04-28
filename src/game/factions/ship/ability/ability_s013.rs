use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS013{}
impl ShipAbility for AbilityS013 {
    fn get_description(&self) -> &'static str {
        "You do not pay a faction penalty when assigning any [crew] or [tech] Upgrades to this ship.\nYou do not place an Auxiliary Power Token beside your ship when using the \"Docking\" or \"Launching\" Actions."
    }
}