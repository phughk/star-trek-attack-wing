use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS263{}
impl ShipAbility for AbilityS263 {
    fn get_description(&self) -> &'static str {
        "<b>WHEN THIS SHIP SUFFERS DAMAGE FROM AN OBSTACLE OR MINEFIELD: </b>\n\nReduce the damage by 1."
    }
}