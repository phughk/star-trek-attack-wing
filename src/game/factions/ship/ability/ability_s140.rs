use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS140{}
impl ShipAbility for AbilityS140 {
    fn get_description(&self) -> &'static str {
        "You do not lose your \"Perform Actions\" step or take damage when you overlap a planet, obstacle or ship base. You may roll defense dice against any minefield."
    }
}