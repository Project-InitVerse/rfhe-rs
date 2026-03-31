use rfhe_versionable::VersionsDispatch;

use crate::shortint::wopbs::WopbsKey;

#[derive(VersionsDispatch)]
pub enum WopbsKeyVersions {
    V0(WopbsKey),
}
