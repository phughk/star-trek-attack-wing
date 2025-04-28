use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS025{}
impl ShipAbility for AbilityS025 {
    fn get_description(&self) -> &'static str {
        "During the Roll Attack Dice step, if there is a [battlestations] Token beside your ship, you may discard that Token to re-roll all of your attack dice."
    }
}