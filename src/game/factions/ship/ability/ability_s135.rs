use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS135{

}

impl ShipAbility for AbilityS135 {
    fn get_description(&self) -> &'static str {
        "When attacking with your Primary Weapon, if you inflict at least 1 Critical Damage to the enemy ship's hull, that ship must also discard 1 [crew] Upgrade (of its choice)."
    }
}