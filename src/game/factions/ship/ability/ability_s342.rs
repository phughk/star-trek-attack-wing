use crate::game::factions::ship::ability::ShipAbility;

pub struct AbilityS342 { }

impl ShipAbility for AbilityS342 {
    fn get_description(&self) -> &'static str {
        "Federation [weapon] Upgrades equipped to this ship cost -1 SP.\n<b>WHEN ATTACKING WITH A PHOTON TORPEDOS UPGRADE, DURING THE MODIFY ATTACK DICE STEP:</b>\nDestroy 1 Active Shield on the defending ship."
    }
}