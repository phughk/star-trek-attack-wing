use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS296{}
impl ShipAbility for AbilityS296 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Target a [tech] or [weapon] Upgrade equipped to an opposing ship within Range 1-2.\nEach player rolls 5 attack dice. If you roll more [battlestations] than the controller of the target Upgrade, steal the target Upgrade even if it exceeds this ships restrictions."
    }
}