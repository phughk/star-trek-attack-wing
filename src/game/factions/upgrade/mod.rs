mod part_1;
mod part_2;
mod part_3;

pub struct StawUpgrade {
    pub upgrade_type: &'static str,
    pub id: &'static str,
    pub set: &'static [&'static str],
    pub name: &'static str,
    pub cost: u8,
    pub text: &'static str,
    pub factions: &'static [&'static str],
    pub constraints: &'static [UpgradeConstraint],
}

pub enum UpgradeConstraint {
    FrontArc,
    RearArc,
    OnePerShip,
    HullConstraint(&'static str),
    ShipKlingon,
    Attack(&'static str),
    /// Range start to end inclusive; Single values are identical number
    Range(u8, u8),
    Unique,
    CaptainKlingon,
    ShipLimit,
    ShipLimitNumber(&'static str),
    ShipFederation,
    Alliance,
    Skill(i32),
    ShipDominion,
    Arc360,
    ShipVulcan,
    CaptainVulcan,
    CaptainBorg,
    ShipBorg,
    CaptainFederation,
    CaptainIndependent,
    /// These cards need to be revisited to handle properly
    Complex,
    AttackConstraint(&'static str),
    ShipIndependent,
    Talents(&'static str),
    Specialisation,
    CaptainFerengi,
    CountsAsUpgrade,
    CaptainDominion,
    CostRomulan(&'static str),
    CaptainRomulan,
    RangeOther(&'static str),
    Banned,
    FactionPenalty(i32),
}



