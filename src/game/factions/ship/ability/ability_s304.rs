use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS304{}

impl ShipAbility for AbilityS304 {
    fn get_description(&self) -> &'static str {
        "ATTACK SQUADRON TOKENS: 5\r\n<b>ACTION:</b> Place 2 [time] Tokens on this card.\nPerform an [sensor-echo] action using a 1 [straight] Maneuver Template as a Free Action, even if this Attack Squadron is not Cloaked."
    }
}