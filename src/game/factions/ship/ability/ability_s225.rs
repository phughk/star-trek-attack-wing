use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS225{}
impl ShipAbility for AbilityS225 {
    fn get_description(&self) -> &'static str {
        "Attack Squadron Tokens: 5\n\nIf you attack during a round in which you performed a ([come-about]) Come About Maneuver, during the Modify Attack Dice step, you may re-roll all of your blank results."
    }
}