use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityEmpty {

}

impl ShipAbility for AbilityEmpty {
    fn get_description(&self) -> &'static str {
        ""
    }
}