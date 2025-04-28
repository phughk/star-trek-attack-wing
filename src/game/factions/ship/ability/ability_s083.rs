use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS083{}
impl ShipAbility for AbilityS083 {
    fn get_description(&self) -> &'static str {
        "After you attack while Cloaked, you may place an Auxiliary Power Token beside your ship to keep your [cloak] Token from flipping to its red side."
    }
}