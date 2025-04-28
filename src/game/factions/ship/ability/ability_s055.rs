use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS055 {

}

impl ShipAbility for AbilityS055 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> Place a [battlestations] Token beside your ship.  Each time you defend this round, roll +1 defense die."
    }
}