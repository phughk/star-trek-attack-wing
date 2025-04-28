use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS272 {}
impl ShipAbility for AbilityS272 {
    fn get_description(&self) -> &'static str {
        "\n<b>WHEN DEFENDING:</b>\n\nCancel 1 [hit]"
    }
}