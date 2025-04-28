use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS067{
    
}

impl ShipAbility for AbilityS067 {
    fn get_description(&self) -> &'static str {
        "Instead of making a normal attack with your Primary Weapon,  you may fire in any direction at up to 2 enemy ships at Range 1 with 4 attack dice against each ship."
    }
}