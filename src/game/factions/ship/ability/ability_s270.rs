use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS270{}
impl ShipAbility for AbilityS270 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b>Target all Cloaked friendly ships within range 1-2.\n\nPlace an [battlestations] Token next to all target ships."
    }
}