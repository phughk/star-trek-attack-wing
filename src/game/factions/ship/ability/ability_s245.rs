use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS245{}
impl ShipAbility for AbilityS245 {
    fn get_description(&self) -> &'static str {
        "If you attack a ship that is in the forward firing arc of another friendly ship, you gain +1 attack die."
    }
}