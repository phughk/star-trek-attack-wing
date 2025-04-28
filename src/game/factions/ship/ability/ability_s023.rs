use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS023{}
impl ShipAbility for AbilityS023 {
    fn get_description(&self) -> &'static str {
        "When attacking, if your ship is not in the target ship's forward firing arc, add +1 attack die."
    }
}