use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS274{}

impl ShipAbility for AbilityS274 {
    fn get_description(&self) -> &'static str {
        "\n<b>WHEN DEFENDING:</b>\n\nThe attacking ship rolls -1 attack die."
    }
}