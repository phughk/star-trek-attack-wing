use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS154{}
impl ShipAbility for AbilityS154 {
    fn get_description(&self) -> &'static str {
        "Instead of making a normal attack, you may make an attack with 2 dice from your rear firing arc. (Initial Captain skill 6)"
    }
}