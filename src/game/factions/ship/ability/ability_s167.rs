use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS167{}
impl ShipAbility for AbilityS167 {
    fn get_description(&self) -> &'static str {
        "After you move, you may perform an [evade] Action as a free Action. If you do so, you cannot attack during this round."
    }
}