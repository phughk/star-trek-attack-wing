pub struct StawGameConfig {
    pub players: Vec<PlayerConfig>,
    pub min_points: Option<usize>,
    pub max_points: Option<usize>,
}

pub struct PlayerConfig {
    pub name: Option<String>,
    pub color: Option<String>,
    pub id: Option<String>,
    pub team: Option<u8>,
}
