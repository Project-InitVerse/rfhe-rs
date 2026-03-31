use rfhe_versionable::VersionsDispatch;

use crate::integer::wopbs::WopbsKey;

#[derive(VersionsDispatch)]
pub enum WopbsKeyVersions {
    V0(WopbsKey),
}
