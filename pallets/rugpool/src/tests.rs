use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn pooling_locks_funds() {
	new_test_ext().execute_with(|| {
		let original = Balances::free_balance(&1);
		assert_ok!(RugPool::pool(Origin::signed(1), 10));
		assert_eq!(Balances::usable_balance(&1), original - 10);
	});
}

#[test]
fn rug_can_be_pulled_once() {
	new_test_ext().execute_with(|| {
		assert_ok!(RugPool::pool(Origin::signed(1), 10));
		assert_ok!(RugPool::pull_rug(Origin::signed(1)));
		assert_noop!(
			RugPool::pull_rug(Origin::signed(1)),
			Error::<Test>::PoolRugged
		);
	});
}
