use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS171{}
impl ShipAbility for AbilityS171 {
    fn get_description(&self) -> &'static str {
        "Instead of making a normal attack, you may spend a [scan] Token to attack 2 different ships with your Primary Weapon at -2 attack dice each."
    }
}