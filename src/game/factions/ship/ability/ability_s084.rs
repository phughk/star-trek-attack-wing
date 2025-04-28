use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS084{}
impl ShipAbility for AbilityS084 {
    fn get_description(&self) -> &'static str {
        "Each time you suffer damage from an attack, you may place an Auxiliary Power Token beside your ship to reduce the amount of damage from that attack by 1."
    }
}