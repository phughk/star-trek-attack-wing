use crate::game::factions::ship::ShipAbility;

pub struct AbilityS340 {

}

impl ShipAbility for AbilityS340 {
    fn get_description(&self) -> &'static str {
        concat!("<b>ACTIVATION PHASE:</b> Target a friendly ship within Range 1-2 that has an ",
        "[aux] Token beside it and place 2 [time] Tokens on a [crew] Upgrade equipped to this ",
        "ship.\nThe target ship does not skip its Perform Action Step this game round.")
    }
}