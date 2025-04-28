use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS040{}
impl ShipAbility for AbilityS040 {
    fn get_description(&self) -> &'static str {
        "After performing a [sensor-echo] Action, you may Come About (reverse direction).  If you do so, place an Auxiliary Power Token beside your ship."
    }
}