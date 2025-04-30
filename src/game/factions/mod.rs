pub mod ship;
pub mod upgrade;

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

pub enum UpgradeRangeType {}

pub enum UpgradeIcon {}

pub enum UpgradeAttackValue {}

pub enum UpgradeAbility {}

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
    SpecialAction { todo: () },
}
