use super::*;
use crate::{Error, mock::*};
use sp_runtime::traits::{BadOrigin, Zero};
use frame_support::{assert_ok, assert_noop};

#[test]
fn estimation_on_next_session() {
    new_test_ext().execute_with(|| {
        assert!(true);
    });
}
