use crate::AbiEncoded;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

pub trait Callable {
	type EncodedCall;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Encode, Decode, TypeInfo)]
#[repr(transparent)]
pub struct XCVMNetwork(u32);

impl XCVMNetwork {
	pub const PICASSO: XCVMNetwork = XCVMNetwork(1);
	pub const ETHEREUM: XCVMNetwork = XCVMNetwork(2);
}

impl Callable for XCVMNetwork {
	type EncodedCall = AbiEncoded;
}

impl Into<u32> for XCVMNetwork {
	fn into(self) -> u32 {
		self.0
	}
}

impl From<u32> for XCVMNetwork {
	fn from(network: u32) -> Self {
		XCVMNetwork(network)
	}
}
