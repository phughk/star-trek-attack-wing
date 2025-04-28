use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS014{}
impl ShipAbility for AbilityS014 {
    fn get_description(&self) -> &'static str {
        "Before you move, you may change your maneuver to another maneuver on your Maneuver Dial with the same speed. If you do so, place an Auxiliary Power Token beside your ship."
    }
}