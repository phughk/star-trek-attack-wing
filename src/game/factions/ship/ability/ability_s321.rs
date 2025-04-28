use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS321 {}
    
impl ShipAbility for AbilityS321 {
    fn get_description(&self) -> &'static str {
        "<b>COMBAT PHASE:</b> Spend a [scan] Token beside this ship and target an opposing ship within Range 1-2.\n\nDiscard a [scan], [battlestations], [evade], or &[target-lock] Token beside the target ship."
    }
}