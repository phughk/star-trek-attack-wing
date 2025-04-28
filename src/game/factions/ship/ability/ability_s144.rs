use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS144{}
impl ShipAbility for AbilityS144 {
    fn get_description(&self) -> &'static str {
        "You may perform the Action listed on the \"Energy Web\" Upgrade card as a free Action."
    }
}