#![no_std]

extern crate alloc;

mod xcvm;

use alloc::{
	collections::{BTreeMap, VecDeque},
	vec::Vec,
};
pub use prost::{DecodeError, EncodeError, Message};
use xcvm::*;
pub use xcvm_core::*;

#[derive(Clone, Debug)]
pub enum DecodingFailure {
	Protobuf(DecodeError),
	Isomorphism,
}

pub fn decode<
	TNetwork: From<u32>,
	TAbiEncoded: TryFrom<Vec<u8>>,
	TAccount: for<'a> TryFrom<&'a [u8]>,
	TAssets: From<BTreeMap<u32, u128>>,
>(
	buffer: &[u8],
) -> Result<XCVMProgram<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>, DecodingFailure>
{
	Program::decode(buffer)
		.map_err(DecodingFailure::Protobuf)
		.and_then(|x| TryInto::try_into(x).map_err(|_| DecodingFailure::Isomorphism))
}

pub fn encode<
	TNetwork: Into<u32>,
	TAbiEncoded: Into<Vec<u8>>,
	TAccount: AsRef<[u8]>,
	TAssets: Into<BTreeMap<u32, u128>>,
>(
	program: XCVMProgram<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>,
) -> Vec<u8> {
	Program::encode_to_vec(&program.into())
}

impl From<u128> for U128 {
	fn from(x: u128) -> Self {
		U128 { bytes: x.to_le_bytes().to_vec() }
	}
}

impl TryInto<u128> for U128 {
	type Error = ();
	fn try_into(self) -> Result<u128, Self::Error> {
		Ok(u128::from_le_bytes(TryInto::<[u8; 16]>::try_into(self.bytes).map_err(|_| ())?))
	}
}

impl<
		TNetwork: Into<u32>,
		TAbiEncoded: Into<Vec<u8>>,
		TAccount: AsRef<[u8]>,
		TAssets: Into<BTreeMap<u32, u128>>,
	> From<XCVMProgram<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>> for Program
{
	fn from(
		XCVMProgram { instructions }: XCVMProgram<
			XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>,
		>,
	) -> Self {
		Program {
			instructions: Some(Instructions {
				instructions: instructions.into_iter().map(Into::<Instruction>::into).collect(),
			}),
		}
	}
}

impl<
		TNetwork: From<u32>,
		TAbiEncoded: TryFrom<Vec<u8>>,
		TAccount: for<'a> TryFrom<&'a [u8]>,
		TAssets: From<BTreeMap<u32, u128>>,
	> TryFrom<Program> for XCVMProgram<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>>
{
	type Error = ();
	fn try_from(Program { instructions }: Program) -> Result<Self, Self::Error> {
		instructions
			.map(|Instructions { instructions }| {
				Ok(XCVMProgram {
					instructions: instructions
						.into_iter()
						.map(TryInto::<XCVMInstruction<_, _, _, _>>::try_into)
						.collect::<Result<VecDeque<_>, _>>()?,
				})
			})
			.unwrap_or(Err(()))
	}
}

impl<
		TNetwork: Into<u32>,
		TAbiEncoded: Into<Vec<u8>>,
		TAccount: AsRef<[u8]>,
		TAssets: Into<BTreeMap<u32, u128>>,
	> From<XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>> for Instruction
{
	fn from(instruction: XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>) -> Self {
		Instruction {
			instruction: Some(match instruction {
				XCVMInstruction::Transfer(destination, assets) =>
					instruction::Instruction::Transfer(Transfer {
						destination: Some(Account { addressed: destination.as_ref().to_vec() }),
						assets: assets
							.into()
							.into_iter()
							.map(|(asset, amount)| (asset, amount.into()))
							.collect(),
					}),
				XCVMInstruction::Call(payload) =>
					instruction::Instruction::Call(Call { payload: payload.into() }),
				XCVMInstruction::Spawn(network, assets, program) =>
					instruction::Instruction::Spawn(Spawn {
						network: network.into(),
						assets: assets
							.into()
							.into_iter()
							.map(|(asset, amount)| (asset, amount.into()))
							.collect(),
						program: program.into_iter().map(Into::into).collect(),
					}),
			}),
		}
	}
}

impl<
		TNetwork: From<u32>,
		TAbiEncoded: TryFrom<Vec<u8>>,
		TAccount: for<'a> TryFrom<&'a [u8]>,
		TAssets: From<BTreeMap<u32, u128>>,
	> TryFrom<Instruction> for XCVMInstruction<TNetwork, TAbiEncoded, TAccount, TAssets>
{
	type Error = ();
	fn try_from(Instruction { instruction }: Instruction) -> Result<Self, Self::Error> {
		instruction
			.map(|instruction| match instruction {
				instruction::Instruction::Transfer(Transfer {
					destination: Some(Account { addressed }),
					assets,
				}) => Ok(XCVMInstruction::Transfer(
					(&addressed[..]).try_into().map_err(|_| ())?,
					assets
						.into_iter()
						.map(|(asset, amount)| Ok((asset, amount.try_into()?)))
						.collect::<Result<BTreeMap<u32, u128>, ()>>()?
						.into(),
				)),
				instruction::Instruction::Call(Call { payload }) =>
					Ok(XCVMInstruction::Call(payload.try_into().map_err(|_| ())?)),
				instruction::Instruction::Spawn(Spawn { network, assets, program }) =>
					Ok(XCVMInstruction::Spawn(
						network.into(),
						assets
							.into_iter()
							.map(|(asset, amount)| Ok((asset, amount.try_into()?)))
							.collect::<Result<BTreeMap<u32, u128>, ()>>()?
							.into(),
						program
							.into_iter()
							.map(TryInto::try_into)
							.collect::<Result<VecDeque<_>, _>>()?,
					)),
				instruction::Instruction::Transfer(Transfer { destination: None, .. }) => Err(()),
			})
			.unwrap_or(Err(()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloc::vec;
	use xcvm_core::{AbiEncoded, XCVMNetwork, XCVMProgramBuilder, XCVMProtocol};

	#[test]
	fn type_isomorphism() {
		struct DummyProtocol;
		impl XCVMProtocol<XCVMNetwork> for DummyProtocol {
			type Error = ();
			fn serialize(&self, network: XCVMNetwork) -> Result<AbiEncoded, ()> {
				match network {
					XCVMNetwork::PICASSO => Ok(AbiEncoded::from(vec![0xCA, 0xFE, 0xBA, 0xBE])),
					XCVMNetwork::ETHEREUM => Ok(AbiEncoded::from(vec![0xDE, 0xAD, 0xC0, 0xDE])),
					_ => Err(()),
				}
			}
		}

		let program = || -> Result<_, ()> {
			XCVMProgramBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, _, Vec<u8>, BTreeMap<u32, u128>>,
			>::from(XCVMNetwork::PICASSO)
			.call(DummyProtocol)?
			.spawn(XCVMNetwork::ETHEREUM, BTreeMap::from([(0x1337, 20_000)]), |child| {
				Ok(child
					.call(DummyProtocol)?
					.transfer(vec![0xBE, 0xEF], BTreeMap::from([(0, 10_000)])))
			})
		}()
		.expect("valid program");

		// f^-1 . f = id
		assert_eq!(
			Ok(program.instructions.clone()),
			program
				.instructions
				.into_iter()
				.map(Into::<Instruction>::into)
				.map(TryFrom::<Instruction>::try_from)
				.collect::<Result<VecDeque<_>, _>>()
		);
	}

	#[test]
	fn encoding_isomorphism() {
		let program = || -> Result<_, ()> {
			Ok(XCVMProgramBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, _, Vec<u8>, BTreeMap<u32, u128>>,
			>::from(XCVMNetwork::PICASSO)
			.spawn::<_, ()>(XCVMNetwork::ETHEREUM, BTreeMap::from([(0x1337, 20_000)]), |child| {
				Ok(child.transfer(vec![0xBE, 0xEF], BTreeMap::from([(0, 10_000)])))
			})?
			.build())
		}()
		.expect("valid program");
		assert_eq!(program.clone(), decode(&encode(program)).expect("must decode"));
	}

	#[test]
	fn test_program() {
		// Transfer to Alice/Bob and redispatch the same program from itself, without the redispatch
		let program = || -> Result<_, ()> {
			Ok(XCVMProgramBuilder::<
				XCVMNetwork,
				XCVMInstruction<XCVMNetwork, _, [u8; 32], BTreeMap<u32, u128>>,
			>::from(XCVMNetwork::PICASSO)
			.transfer(
				hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d")
					.expect("valid")
					.try_into()
					.expect("32 bytes"),
				BTreeMap::from([(XCVMAsset::PICA.into(), 1337000000000000)]),
			)
			.transfer(
				hex::decode("8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48")
					.expect("valid")
					.try_into()
					.expect("32 bytes"),
				BTreeMap::from([(XCVMAsset::PICA.into(), 1336000000000000)]),
			)
      .call_raw(hex::decode("44020d020a80010a3e0a3c0a221220d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d1216080112120a1000901092febf040000000000000000000a3e0a3c0a2212208eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a481216080112120a1000806bbd15bf04000000000000000000").expect("valid call").into())
		  .build())
		}()
		.expect("valid program");
		assert_eq!(
      "0a90020a3e0a3c0a221220d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d1216080112120a1000901092febf040000000000000000000a3e0a3c0a2212208eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a481216080112120a1000806bbd15bf040000000000000000000a8d011a8a010a870144020d020a80010a3e0a3c0a221220d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d1216080112120a1000901092febf040000000000000000000a3e0a3c0a2212208eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a481216080112120a1000806bbd15bf04000000000000000000",
      hex::encode(encode(program))
    );
	}
}
