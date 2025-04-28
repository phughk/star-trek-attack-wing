use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS174{}
impl ShipAbility for AbilityS174 {
    fn get_description(&self) -> &'static str {
        "<b>ACTION:</b> When attacking this round, gain +1 attack die. When defending this round, roll one less defense die."
    }
}