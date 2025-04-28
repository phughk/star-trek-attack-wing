use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS329 {}

impl ShipAbility for AbilityS329 {
    fn get_description(&self) -> &'static str {
        "<b>COMBAT PHASE:</b> If this ship is cloaked:\nPlace a [battlestations],[scan],OR [evade] Token beside this ship OR increase the Captain Skill of the Captain equipped to this ship by 3 this Combat Phase"
    }
}