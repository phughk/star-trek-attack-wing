use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS316 {
    
}

impl ShipAbility for AbilityS316 {
    fn get_description(&self) -> &'static str {
        "You may equip a [crew] Upgrade as the Captain of this ship. If you do, it counts as a Captain instead of a [crew] Upgrade and its Captain Skill is its Printed SP Cost +3.\n----------------------------------\nYou may not equip an Admiral Card to this ship."
    }
}