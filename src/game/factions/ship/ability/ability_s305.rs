use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS305 {

}

impl ShipAbility for AbilityS305 {
    fn get_description(&self) -> &'static str {
        "ATTACK SQUADRON TOKENS: 5\r\n<b>ACTION:</b> Place 3 [time] Tokens on this card.\n\nEquip a [squadron] Upgrade that was discarded from this Attack Squadron to this Attack Squadron."
    }
}