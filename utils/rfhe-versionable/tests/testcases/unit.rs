use static_assertions::assert_impl_all;

use rfhe_versionable::Version;

#[derive(Version)]
pub struct MyUnit;

fn main() {
    assert_impl_all!(MyUnit: Version);
}
