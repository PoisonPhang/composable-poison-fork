<<<<<<< HEAD
use sp_runtime::{DispatchError, Permill};

use crate::{dex::Orderbook, loans::PriceStructure};

/// An object from which we can initiate liquidations from.
/// Does not cares if liquidation was completed or not, neither can reasonably provide that
/// information. Off-chain can join relevant ids if needed.
pub trait Liquidation {
	type AssetId;
	type Balance;
	type AccountId;
	type LiquidationId;
	type GroupId;

	/// Initiate a liquidation, this operation should be executed as fast as possible.
	fn liquidate(
		source_account: &Self::AccountId,
		source_asset_id: Self::AssetId,
		source_asset_price: PriceStructure<Self::GroupId, Self::Balance>,
		target_asset_id: Self::AssetId,
		target_account: &Self::AccountId,
		total_amount: Self::Balance,
	) -> Result<Self::LiquidationId, DispatchError>;
}

impl<T: Orderbook> Liquidation for T {
	type AssetId = <Self as Orderbook>::AssetId;
	type Balance = <Self as Orderbook>::Balance;
	type AccountId = <Self as Orderbook>::AccountId;
	type LiquidationId = <Self as Orderbook>::OrderId;
	type GroupId = <Self as Orderbook>::GroupId;

	fn liquidate(
		source_account: &Self::AccountId,
		source_asset_id: Self::AssetId,
		_source_asset_price: PriceStructure<Self::GroupId, Self::Balance>,
		target_asset_id: Self::AssetId,
		_target_account: &Self::AccountId,
		total_amount: Self::Balance,
	) -> Result<Self::LiquidationId, DispatchError> {
		<T as Orderbook>::market_sell(
			source_account,
			source_asset_id,
			target_asset_id,
			total_amount,
			Permill::from_perthousand(0),
		)
	}
}
=======
use sp_runtime::{DispatchError, Permill};

use crate::{dex::{LimitOrderbook,}, defi::{SellTrait, DeFiTrait}, auction::AuctionStepFunction};

pub trait Liquidate {
	type AssetId;
	type Balance;
	type AccountId;
	type LiquidationId;
	type AmmConfiguration;
	
	fn initiate_liquidation(
		source_account: &Self::AccountId,
		source_asset_id: Self::AssetId,
		source_asset_price: Self::Balance,
		target_asset_id: Self::AssetId,
		target_account: &Self::AccountId,
		total_amount: Self::Balance,
	) -> Result<Self::LiquidationId, DispatchError>;
}
>>>>>>> dz/obdex
