use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS087{}
impl ShipAbility for AbilityS087 {
    fn get_description(&self) -> &'static str {
        "When you initiate an attack at Range 1, while Cloaked, your opponent rolls -1 defense die. \n\nYou cannot deploy this card to the same fleet as \"Assimilated Vessel 80279.\""
    }
}