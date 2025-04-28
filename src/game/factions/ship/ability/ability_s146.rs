use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS146{

}
impl ShipAbility for AbilityS146 {
    fn get_description(&self) -> &'static str {
        "When you attack with your Primary Weapon, if you inflict at least 3 damage, place an Auxiliary Power Token beside the target ship."
    }
}