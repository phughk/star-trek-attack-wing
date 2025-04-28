use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS302 {}

impl ShipAbility for AbilityS302 {
    fn get_description(&self) -> &'static str {
        "<b>AFTER THIS SHIP DEALS NO DAMAGE WITH AN ATTACK:</b> Target the defending ship. \nPlace a [battlestations] Token beside this ship and perform a &[target-lock] Action as a free Action on the target ship."
    }
}