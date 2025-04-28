use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS160{}
impl ShipAbility for AbilityS160 {
    fn get_description(&self) -> &'static str {
        "When initiating an attack while Cloaked, you may fire Plasma Torpedoes without needing a Target Lock."
    }
}