use composable_traits::defi::CurrencyPair;
use frame_support::assert_ok;
use pallet_pablo::PoolInitConfiguration;
use primitives::currency::CurrencyId;
use sp_runtime::Permill;

use super::runtime::{Balance, Pablo, PoolId};
use crate::mock::runtime::{AccountId, BlockNumber};

pub fn create_usdt_usdc_pool() -> PoolId {
	let usdt_amount = 1_000_000_000 * CurrencyId::unit::<Balance>();
	let usdc_amount = 1_000_000_000 * CurrencyId::unit::<Balance>();
	let assets = CurrencyPair::new(CurrencyId::USDT, CurrencyId::USDC);
	let amounts = vec![usdt_amount, usdc_amount];
	create_pool(assets, amounts, 100, Permill::zero())
}

fn create_pool(
	assets: CurrencyPair<CurrencyId>,
	// TODO(saruman9): add amount to a pools
	_amounts: Vec<Balance>,
	amplification_coefficient: u16,
	fee: Permill,
) -> PoolId {
	let config = PoolInitConfiguration::<AccountId, CurrencyId, BlockNumber>::StableSwap {
		// TODO(saruman9): create users
		owner: 1,
		pair: assets,
		amplification_coefficient,
		fee,
	};
	let pool_id = Pablo::do_create_pool(config);
	assert_ok!(pool_id);
	pool_id.unwrap()
}
