use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS229{}
impl ShipAbility for AbilityS229 {
    fn get_description(&self) -> &'static str {
        "If your ship has an opposing ship target locked, during the Activation Phase, you may switch the target lock to a different ship within Range 1-3 as a free Action."
    }
}