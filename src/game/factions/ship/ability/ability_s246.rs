use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS246{}
impl ShipAbility for AbilityS246 {
    fn get_description(&self) -> &'static str {
        "When defending, if your ship is Cloaked, you may re-roll up to 2 of your defense dice."
    }
}