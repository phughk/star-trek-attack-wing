use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS294{}

impl ShipAbility for AbilityS294 {
    fn get_description(&self) -> &'static str {
        "Other ships within Range 1-2 cannot be removed from the play area unless they are destroyed."
    }
}