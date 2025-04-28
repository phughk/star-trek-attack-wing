use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS308 {

}

impl ShipAbility for AbilityS308 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN THIS SHIP SUFFERS DAMAGE TO ITS HULL:</b> Once per game, if this ship is Cloaked:\n\nThis ship may perform an attack with its Primary Weapon."
    }
}