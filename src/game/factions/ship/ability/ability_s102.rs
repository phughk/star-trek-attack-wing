use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS102{}
impl ShipAbility for AbilityS102 {
    fn get_description(&self) -> &'static str {
        "After you move, if you are within Range 1 of a friendly ship, you may immediately perform one of the Actions listed on your Action Bar as a free Action."
    }
}