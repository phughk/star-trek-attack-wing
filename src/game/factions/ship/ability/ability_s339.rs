use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS339 {

}

impl ShipAbility for AbilityS339 {
    fn get_description(&self) -> &'static str {
        "<b>FREE ACTION:</b> Place an [aux] Token beside this ship and target all friendly Federation ships with a Hull Value of 3 or less within Range 1-2.\nThe target ships roll +2 defense dice and may convert 1 [blank] into 1 [battlestations] the next time they defend this game round."
    }
}