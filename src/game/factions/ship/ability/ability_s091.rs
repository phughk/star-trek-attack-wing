use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS091{}
impl ShipAbility for AbilityS091 {
    fn get_description(&self) -> &'static str {
        "You may fire a Torpedo at an enemy ship without needing a Target Lock.  If you do so, place an Auxiliary Power Token beside your ship."
    }
}