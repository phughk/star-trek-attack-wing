use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS060{}
impl ShipAbility for AbilityS060 {
    fn get_description(&self) -> &'static str {
        "When attacking, you may convert 1 of your [crit] results into a [hit] result.  If you do so, you may then convert 1 of your blank results into a [hit] result."
    }
}