use crate::game::stage::deploy::DeployState;

pub struct BuildState {}

impl BuildState {
    pub fn deploy(self) -> DeployState {
        DeployState {}
    }
}
