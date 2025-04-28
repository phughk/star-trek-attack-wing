use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS242{}

impl ShipAbility for AbilityS242 {
    fn get_description(&self) -> &'static str {
        "If there are only Federation cards assigned to this ship, add the [scan] and [battlestations] icons to the Action Bar. \n\nThis ship cannot be in the same fleet as Kruge's Bird-of-Prey."
    }
}