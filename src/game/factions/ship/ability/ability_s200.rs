use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS200{}
impl ShipAbility for AbilityS200{
    fn get_description(&self) -> &'static str {
        "The U.S.S. Enterprise can perform an Action listed on its Action Bar while it has an Auxiliary Power Token."
    }
}