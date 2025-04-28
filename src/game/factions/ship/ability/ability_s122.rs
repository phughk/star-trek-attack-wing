use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS122{}
impl ShipAbility for AbilityS122 {
    fn get_description(&self) -> &'static str {
        "After performing a 5 [forward] Maneuver, if there are no enemy ships in your forward firing arc, you may perform an [evade] Action as a free Action."
    }
}

