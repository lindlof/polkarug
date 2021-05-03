//! Benchmarking setup for pallet-rugpool

use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::{boxed::Box, vec, vec::Vec};

#[allow(unused)]
use crate::Module as RugPool;

benchmarks! {
	pull_rug {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller))
	verify {
		assert_eq!(RugPulled::<T>::get(), Some(true));
	}
}

impl_benchmark_test_suite!(RugPool, crate::mock::new_test_ext(), crate::mock::Test,);
