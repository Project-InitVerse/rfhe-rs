use crate::core_crypto::commons::dispersion::StandardDev;
use rfhe_versionable::VersionsDispatch;

#[derive(VersionsDispatch)]
pub enum StandardDevVersions {
    V0(StandardDev),
}
