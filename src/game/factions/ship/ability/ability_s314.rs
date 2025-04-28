use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS314 {}

impl ShipAbility for AbilityS314 {
    fn get_description(&self) -> &'static str {
        "<b><font size=\"-1\">WHEN A FRIENDLY SHIP WITHIN RANGE 1 IS DEFENDING:</font></b> The friendly ship may roll +1 defense die. If that friendly ship is an (<i>Andorian Battle Cruiser</i>), it may also convert one [blank] result into one [evade] result. If the attacking ship is a Vulcan Ship, that friendly ship may also add one [evade]."
    }
}
