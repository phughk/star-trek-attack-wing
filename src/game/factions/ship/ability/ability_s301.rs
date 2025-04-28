use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS301{}
impl ShipAbility for AbilityS301 {
    fn get_description(&self) -> &'static str {
        "<b>AFTER THIS SHIP PERFORMS A  &[target-lock] ACTION:</b> Target a friendly Jem'Hadar Attack Ship within Range 1.\n\nThe target ship may perform a  &[target-lock] Action as a Free Action."
    }
}