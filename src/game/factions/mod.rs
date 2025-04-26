mod federation;
mod klingon;
mod vulcan;
mod bajoran;
mod independent;
mod ferengi;
mod kazon;
mod xindi;
mod romulan;
mod dominion;
mod borg;
mod species_8472;
mod mirror_universe;
mod q_continuum;

pub enum StawFaction {
    All,
    Bajoran,
    Borg,
    Dominion,
    /// Federation includes subfactions Vulcan and Bajoran
    Federation,
    Ferengi,
    /// Independents are Ferengi, Kazon, and Xindi
    Independent,
    Kazon,
    Klingon,
    MirrorUniverse,
    QContinuum,
    Romulan,
    Species8472,
    Vulcan,
    Xindi,
}

pub struct StawShipArchetype {
    pub name: &'static str,
    pub unique: bool,
    pub class: &'static str,
    pub factions: &'static [StawFaction],
    pub primary_weapon: u8,
    pub agility: u8,
    pub hull: u8,
    pub shields: u8,
    pub ship_type: ShipType,
    pub upgrade_bar: &'static [UpgradeType],
    pub special_ability_description: &'static str,
    pub special_ability: ShipSpecialAbility,
    pub action_bar: &'static [ActionType],
    pub squad_point_cost: u8,
}

pub struct UpgradeArchetype {
    pub name: &'static str,
    pub ability_description: &'static str,
    pub ability: UpgradeAbility,
    pub attack_value: UpgradeAttackValue,
    pub range: UpgradeRangeType,
    pub upgrade_icon: UpgradeIcon,
    pub faction: StawFaction,
    pub squad_point_cost: u8,
    pub unique: bool,
    pub restrictions: UpgradeRestriction,
}

pub enum UpgradeRestriction {
    /// No restrictions
    None,
    /// Front facing 90
    PrimaryFiringArc,
    /// Rear facing 90
    SecondaryFiringArc,
    /// 360
    AllRoundFiringArc,
}

pub enum UpgradeRangeType {

}

pub enum UpgradeIcon {

}

pub enum UpgradeAttackValue {

}

pub enum UpgradeAbility {

}

pub enum ShipType {
    DeepSpace9,
    UssVoyager,
    EnterpriseE,
}

pub enum UpgradeType {
    /// Looks like 3 tadpoles
    Squadron,
    /// Arrow-like badge (like in Commandos)
    Crew,
    /// 3 mines? star-spike like dots
    Weapon,
    /// Spanner and wrench
    Tech,
    /// Beefy invader type dude
    Borg,
    /// Some weird rules around this but looks like an alembic
    EliteTalent,
}

pub enum ActionType {
    /// Looks like jellyfish
    EvasiveManeuvers,
    /// Looks like wifi icon
    Scan,
    /// Looks like squid game helmet or circle with dots inside
    BattleStations,
    /// Looks like light bulb
    Cloak,
    /// Looks like a square sheet of metal that is slightly curved with 3 lines at top
    SensorEcho,
    /// Looks like a pane of glass on a radar
    AcquireTargetLock,
    /// Looks like DNA double helix
    Regenerate,
    /// There is no icon. Only "Action: " description
    SpecialAction{todo: ()}
}

pub enum ShipSpecialAbility {
    DeepSpace9_1,
    DeepSpace9_2,
}