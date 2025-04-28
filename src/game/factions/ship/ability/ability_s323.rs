use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS323 {}

impl ShipAbility for AbilityS323 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN ATTACKING:</b> If this ship is Cloaked:  This ship rolls +1 attack die this attack.\n-----------------------------------\n<b>WHEN ATTACKING: </b> If this ship is not within the Primary Firing Arc of the defending ship.  Convert 1 [battlestations] into 1 [crit] and all other [battlestations] into [hit]."
    }
}