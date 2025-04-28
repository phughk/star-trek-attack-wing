use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS307 {
    
}

impl ShipAbility for AbilityS307 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING:</b> If this ship is Cloaked and not in the Primary Firing Arc of the defending ship.\n\nThis ship rolls +1 attack die this attack and the defending ship rolls -1 defence die this attack."
    }
}