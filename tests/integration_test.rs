// Integration test for SigmaOS-mintlocale
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_mintlocale as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
