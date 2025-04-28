use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS282 {}
impl ShipAbility for AbilityS282 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING:</b> If this ship is Cloaked, place an [aux] Token beside it.\n\nDisable up to 2 Shields on the defending ship."
    }
}