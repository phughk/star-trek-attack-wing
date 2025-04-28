use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS298{}
impl ShipAbility for AbilityS298 {
    fn get_description(&self) -> &'static str {
        "<font size=\"-1\"><b>COMBAT PHASE:</b> If there is an opposing ship in the Primary Firing Arc of this ship and the Primary Firing Arc of another Ferengi ship with a Ferengi Captain equipped to it, target that friendly ship.  This ship attack first this game round and the target ship attacks second this game round.</font>"
    }
}