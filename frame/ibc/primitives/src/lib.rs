#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use codec::{Decode, Encode};
use composable_traits::xcm::assets::XcmAssetLocation;
use frame_support::{weights::Weight, RuntimeDebug};
use ibc::{
	applications::transfer::{
		error::Error as Ics20Error, msgs::transfer::MsgTransfer, PrefixedCoin, VERSION,
	},
	core::{
		ics02_client::client_type::ClientType,
		ics04_channel::{
			channel::{ChannelEnd, Order},
			msgs::acknowledgement::Acknowledgement,
			packet::{Packet, Sequence},
		},
		ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId},
		ics26_routing::context::ModuleOutputBuilder,
	},
	signer::Signer, 	timestamp::Timestamp,

};
use scale_info::{prelude::format, TypeInfo};
use sp_std::{prelude::*, str::FromStr};
use alloc::string::{String, ToString};
use xcm::v1::{Junction, Junctions, MultiLocation};
#[cfg(not(feature = "std"))]
use sp_std::vec::Vec;

pub mod runtime_interface;

pub struct SendPacketData {
	/// packet data
	pub data: Vec<u8>,
	/// Needed only when packet is been sent to a parachain, this should be the parachain id in
	/// that case.
	pub revision_number: Option<u64>,
	/// Block height on the counterparty chain when this packet should be invalidated.
	pub timeout_height: u64,
	/// Timestamp on counterparty chain when this packet should be invalidated
	/// This value should be in nano seconds
	pub timeout_timestamp: u64,
	/// port id as utf8 string bytes
	pub port_id: Vec<u8>,
	/// channel id as utf8 string bytes
	pub channel_id: Vec<u8>,
}
#[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct OffchainPacketType {
	pub sequence: u64,
	pub source_port: Vec<u8>,
	pub source_channel: Vec<u8>,
	pub destination_port: Vec<u8>,
	pub destination_channel: Vec<u8>,
	pub data: Vec<u8>,
	pub timeout_height: (u64, u64),
	pub timeout_timestamp: u64,
}

impl From<OffchainPacketType> for Packet {
	fn from(packet: OffchainPacketType) -> Self {
		Self {
			sequence: Sequence::from(packet.sequence),
			source_port: PortId::from_str(
				&String::from_utf8(packet.source_port).unwrap_or_default(),
			)
			.unwrap_or_default(),
			source_channel: ChannelId::from_str(
				&String::from_utf8(packet.source_channel).unwrap_or_default(),
			)
			.unwrap_or_default(),
			destination_port: PortId::from_str(
				&String::from_utf8(packet.destination_port).unwrap_or_default(),
			)
			.unwrap_or_default(),
			destination_channel: ChannelId::from_str(
				&String::from_utf8(packet.destination_channel).unwrap_or_default(),
			)
			.unwrap_or_default(),
			data: packet.data,
			timeout_height: ibc::Height::new(packet.timeout_height.0, packet.timeout_height.1),
			timeout_timestamp: Timestamp::from_nanoseconds(packet.timeout_timestamp)
				.unwrap_or_default(),
		}
	}
}

impl From<Packet> for OffchainPacketType {
	fn from(packet: Packet) -> Self {
		Self {
			sequence: packet.sequence.into(),
			source_port: packet.source_port.to_string().as_bytes().to_vec(),
			source_channel: packet.source_channel.to_string().as_bytes().to_vec(),
			destination_port: packet.destination_port.to_string().as_bytes().to_vec(),
			destination_channel: packet.destination_channel.to_string().as_bytes().to_vec(),
			data: packet.data,
			timeout_height: (
				packet.timeout_height.revision_number,
				packet.timeout_height.revision_height,
			),
			timeout_timestamp: packet.timeout_timestamp.nanoseconds(),
		}
	}
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct IdentifiedChannel {
	pub channel_id: Vec<u8>,
	pub port_id: Vec<u8>,
	/// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channel_end: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct IdentifiedClientState {
	pub client_id: Vec<u8>,
	/// Protobuf encoded `AnyClientState`
	pub client_state: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct IdentifiedConnection {
	pub connection_id: Vec<u8>,
	/// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connection_end: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryClientStateResponse {
	/// Protobuf encoded `AnyClientState`
	pub client_state: Vec<u8>,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryClientStatesResponse {
	pub client_states: Vec<Vec<u8>>,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryConsensusStateResponse {
	pub consensus_state: Vec<u8>,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryConnectionResponse {
	/// Protobuf encoded `ibc::core::ics03_connection::connection::ConnectionEnd`
	pub connection: Vec<u8>,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryChannelResponse {
	/// Protobuf encoded `ibc::core::ics04_channel::connection::ChannelEnd`
	pub channel: Vec<u8>,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryChannelsResponse {
	pub channels: Vec<IdentifiedChannel>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryConnectionsResponse {
	pub connections: Vec<IdentifiedConnection>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryNextSequenceReceiveResponse {
	pub sequence: u64,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketCommitmentResponse {
	pub commitment: Vec<u8>,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct PacketState {
	pub port_id: Vec<u8>,
	pub channel_id: Vec<u8>,
	pub sequence: u64,
	pub data: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketCommitmentsResponse {
	pub commitments: Vec<PacketState>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketAcknowledgementResponse {
	pub ack: Vec<u8>,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketAcknowledgementsResponse {
	pub acks: Vec<PacketState>,
	pub height: u64,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryPacketReceiptResponse {
	pub receipt: bool,
	pub height: u64,
	pub trie_key: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryDenomTraceResponse {
	pub denom: Vec<u8>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct QueryDenomTracesResponse {
	pub denoms: Vec<Vec<u8>>,
	pub next_key: Option<u128>,
	pub total: Option<u64>,
}

#[derive(Clone, codec::Encode, codec::Decode, PartialEq, Eq, Ord, PartialOrd)]
pub struct ConnectionHandshake {
	pub client_state: Vec<u8>,
	pub trie_keys: Vec<Vec<u8>>,
	pub height: u64,
}


#[derive(core::fmt::Debug, Clone, PartialEq, Eq)]
/// Error definition for module
pub enum Error {
	/// Failed to register a new packet
	SendPacketError { msg: Option<String> },
	/// An error involving the connection id
	ConnectionIdError { msg: Option<String> },
	/// An error involving the client id
	ClientIdError { msg: Option<String> },
	/// An error involving channel or port
	ChannelOrPortError { msg: Option<String> },
	/// An error involving Client state
	ClientStateError { msg: Option<String> },
	/// An Error Involving the Timestamp and height
	TimestampOrHeightError { msg: Option<String> },
	/// Failed to register a token transfer packet
	SendTransferError { msg: Option<String> },
	/// Ics20 receive packet processing error
	ReceivePacketError { msg: Option<String> },
	/// Write acknowledgement error
	WriteAcknowledgementError { msg: Option<String> },
	/// Ics20 packet acknowledgement processing error
	AcknowledgementError { msg: Option<String> },
	/// Ics20 packet timeout processing error
	TimeoutError { msg: Option<String> },
	/// Failed to bind port
	BindPortError { msg: Option<String> },
	/// Failed to intialize a new channel
	ChannelInitError { msg: Option<String> },
	/// Failed to decode a value
	DecodingError { msg: Option<String> },
	/// Failed to decode commitment prefix
	ErrorDecodingPrefix,
	/// Some other error
	Other { msg: Option<String> },
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
/// Captures all parameters needed to initialize a channel
pub struct OpenChannelParams {
	/// channel order
	pub order: u8,
	/// connection id as utf8 string bytes
	pub connection_id: Vec<u8>,
	/// counterparty port id as utf8 string bytes
	pub counterparty_port_id: Vec<u8>,
	/// version as utf8 string bytes
	pub version: Vec<u8>,
}

impl TryFrom<&OpenChannelParams> for Order {
	type Error = Error;

	fn try_from(value: &OpenChannelParams) -> Result<Self, Self::Error> {
		match value.order {
			0 => Ok(Order::None),
			1 => Ok(Order::Unordered),
			2 => Ok(Order::Ordered),
			_ => Err(Error::Other { msg: None }),
		}
	}
}

/// Captures the functions modules can use to interact with the ibc pallet
/// Currently allows modules to register packets and crreate channels
pub trait IbcTrait {
	fn client_revision_number(port_id: Vec<u8>, channel_id: Vec<u8>) -> Result<u64, Error>;
	/// Register a packet to be sent
	fn send_packet(data: SendPacketData) -> Result<(), Error>;
	/// Allows a module to open a channel
	fn open_channel(port_id: PortId, channel_end: ChannelEnd) -> Result<ChannelId, Error>;
	/// Modules use this to write acknowledgements into the ibc store
	/// To be used in a successful execution of OnRecvPacket callback
	fn write_acknowlegdement(packet: &Packet, ack: Vec<u8>) -> Result<(), Error>;
	/// These methods are majorly for the ibc token transfer application
	/// We need these here because the implementation of the ics20 requires access to the context
	/// which is defined in pallet-ibc, we cannot import the context in pallet-ibc-transfer because
	/// it would cause a cyclic dependency
	/// Perform an ibc token transfer
	fn send_transfer(data: MsgTransfer<PrefixedCoin>) -> Result<(), Error>;
	/// on receive packet callback for ibc token transfer
	fn on_receive_packet(output: &mut ModuleOutputBuilder, packet: &Packet) -> Result<(), Error>;
	/// on acknowledgement packet callback for ibc token transfer
	fn on_ack_packet(
		output: &mut ModuleOutputBuilder,
		packet: &Packet,
		ack: &Acknowledgement,
	) -> Result<(), Error>;
	/// on timeout packet callback for ibc token transfer
	fn on_timeout_packet(output: &mut ModuleOutputBuilder, packet: &Packet) -> Result<(), Error>;
	#[cfg(feature = "runtime-benchmarks")]
	fn create_client() -> Result<ClientId, Error>;
	#[cfg(feature = "runtime-benchmarks")]
	fn create_connection(client_id: ClientId, connection_id: ConnectionId) -> Result<(), Error>;
}

/// Callback Weight
/// This trait must be implemented by module callback handlers to be able to estimate the weight
/// of the callback function.
pub trait CallbackWeight {
	/// Returns the callback weight for the channel open init ibc message
	fn on_chan_open_init(&self) -> Weight;

	/// Returns the callback weight for the channel open try ibc message
	fn on_chan_open_try(&self) -> Weight;

	/// Returns the callback weight for the channel open acknowledgement ibc message
	fn on_chan_open_ack(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	/// Returns the callback weight for the channel open comfirm ibc message
	fn on_chan_open_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	/// Returns the callback weight for the channel close init ibc message
	fn on_chan_close_init(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	/// Returns the callback weight for the channel close confirm ibc message
	fn on_chan_close_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight;

	/// Returns the callback weight for the receive packet ibc message
	fn on_recv_packet(&self, _packet: &Packet) -> Weight;

	/// Returns the callback weight for the packet acknowledgement ibc message
	fn on_acknowledgement_packet(
		&self,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
	) -> Weight;

	/// Returns the callback weight for the packet timeout ibc message
	fn on_timeout_packet(&self, packet: &Packet) -> Weight;
}

impl CallbackWeight for () {
	fn on_chan_open_init(&self) -> Weight {
		Weight::MAX
	}

	fn on_chan_open_try(&self) -> Weight {
		Weight::MAX
	}

	fn on_chan_open_ack(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		Weight::MAX
	}

	fn on_chan_open_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		Weight::MAX
	}

	fn on_chan_close_init(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		Weight::MAX
	}

	fn on_chan_close_confirm(&self, _port_id: &PortId, _channel_id: &ChannelId) -> Weight {
		Weight::MAX
	}

	fn on_recv_packet(&self, _packet: &Packet) -> Weight {
		Weight::MAX
	}

	fn on_acknowledgement_packet(
		&self,
		_packet: &Packet,
		_acknowledgement: &Acknowledgement,
	) -> Weight {
		Weight::MAX
	}

	fn on_timeout_packet(&self, _packet: &Packet) -> Weight {
		Weight::MAX
	}
}

/// Get port_id from raw bytes
pub fn port_id_from_bytes(port: Vec<u8>) -> Result<PortId, Error> {
	PortId::from_str(&String::from_utf8(port).map_err(|_| Error::DecodingError { msg: None })?)
		.map_err(|_| Error::DecodingError { msg: None })
}

/// Get channel_id from raw bytes
pub fn channel_id_from_bytes(channel: Vec<u8>) -> Result<ChannelId, Error> {
	ChannelId::from_str(
		&String::from_utf8(channel).map_err(|_| Error::DecodingError { msg: None })?,
	)
		.map_err(|_| Error::DecodingError { msg: None })
}

/// Get connection_id from raw bytes
pub fn connection_id_from_bytes(connection: Vec<u8>) -> Result<ConnectionId, Error> {
	ConnectionId::from_str(
		&String::from_utf8(connection).map_err(|_| Error::DecodingError { msg: None })?,
	)
		.map_err(|_| Error::DecodingError { msg: None })
}

/// Get client_id from raw bytes
pub fn client_id_from_bytes(client_id: Vec<u8>) -> Result<ClientId, Error> {
	ClientId::from_str(
		&String::from_utf8(client_id).map_err(|_| Error::DecodingError { msg: None })?,
	)
		.map_err(|_| Error::DecodingError { msg: None })
}

/// Get client_type from raw bytes
pub fn client_type_from_bytes(client_type: Vec<u8>) -> Result<ClientType, Error> {
	ClientType::from_str(
		&String::from_utf8(client_type).map_err(|_| Error::DecodingError { msg: None })?,
	)
		.map_err(|_| Error::DecodingError { msg: None })
}

/// Get trie key by applying the commitment prefix to the path
pub fn apply_prefix(prefix: &[u8], path: Vec<String>) -> Vec<u8> {
	let mut key_path = prefix.to_vec();
	let path = path.iter().flat_map(|val| val.as_bytes()).collect::<Vec<_>>();
	key_path.extend(path);
	key_path
}

pub fn ibc_denom_to_foreign_asset_id(denom: &str) -> XcmAssetLocation {
	let hash = sp_io::hashing::sha2_256(denom.as_bytes()).to_vec();
	XcmAssetLocation::new(MultiLocation {
		parents: 0,
		interior: Junctions::X1(Junction::GeneralKey(hash)),
	})
}

pub fn get_channel_escrow_address(
	port_id: &PortId,
	channel_id: ChannelId,
) -> Result<Signer, Ics20Error> {
	let contents = format!("{}/{}", port_id, channel_id);
	let mut data = VERSION.as_bytes().to_vec();
	data.extend_from_slice(&[0]);
	data.extend_from_slice(contents.as_bytes());

	let hash = sp_io::hashing::sha2_256(&data).to_vec();
	let mut hex_string = hex::encode_upper(hash);
	hex_string.insert_str(0, "0x");
	hex_string.parse::<Signer>().map_err(Ics20Error::signer)
}
