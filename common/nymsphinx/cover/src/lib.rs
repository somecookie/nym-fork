// Copyright 2020 Nym Technologies SA
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::convert::TryFrom;
use std::time;

use rand::{CryptoRng, RngCore};

use crypto::shared_key::new_ephemeral_shared_key;
use crypto::symmetric::stream_cipher;
use nymsphinx_acknowledgements::AckKey;
use nymsphinx_acknowledgements::surb_ack::SURBAck;
use nymsphinx_addressing::clients::Recipient;
use nymsphinx_addressing::nodes::{NymNodeRoutingAddress, NymNodeRoutingAddressError};
use nymsphinx_chunking::fragment::COVER_FRAG_ID;
use nymsphinx_forwarding::packet::MixPacket;
use nymsphinx_params::{
    DEFAULT_NUM_MIX_HOPS, PacketEncryptionAlgorithm, PacketHkdfAlgorithm, PacketMode,
};
use nymsphinx_params::packet_sizes::PacketSize;
use nymsphinx_types::{delays, Error as SphinxError};
use nymsphinx_types::builder::SphinxPacketBuilder;
use topology::{NymTopology, NymTopologyError};

pub const LOOP_COVER_MESSAGE_PAYLOAD: &[u8] = b"The cake is a lie!";
pub const DROP_COVER_MESSAGE_PAYLOAD: &[u8] = b"Follow the white rabbit!";

#[derive(Debug)]
pub enum CoverMessageError {
    NoValidProvidersError,
    InvalidTopologyError,
    SphinxError(SphinxError),
    InvalidFirstMixAddress,
}

impl From<SphinxError> for CoverMessageError {
    fn from(err: SphinxError) -> Self {
        CoverMessageError::SphinxError(err)
    }
}

impl From<NymNodeRoutingAddressError> for CoverMessageError {
    fn from(_: NymNodeRoutingAddressError) -> Self {
        use CoverMessageError::*;
        InvalidFirstMixAddress
    }
}

impl From<NymTopologyError> for CoverMessageError {
    fn from(_: NymTopologyError) -> Self {
        CoverMessageError::InvalidTopologyError
    }
}

pub fn generate_loop_cover_surb_ack<R>(
    rng: &mut R,
    topology: &NymTopology,
    ack_key: &AckKey,
    full_address: &Recipient,
    average_ack_delay: time::Duration,
) -> Result<SURBAck, CoverMessageError>
    where
        R: RngCore + CryptoRng,
{
    Ok(SURBAck::construct(
        rng,
        full_address,
        ack_key,
        COVER_FRAG_ID.to_bytes(),
        average_ack_delay,
        topology,
        None,
    )?)
}


pub fn generate_drop_cover_packet<R>(
    rng: &mut R,
    topology: &NymTopology,
    full_address: &Recipient,
    average_packet_delay: time::Duration,
) -> Result<MixPacket, CoverMessageError>
    where
        R: RngCore + CryptoRng,
{
    // cover message can't be distinguishable from a normal traffic so we have to go through
    // all the effort of key generation, encryption, etc. Note here we are generating shared key
    // with ourselves!
    let (ephemeral_keypair, shared_key) = new_ephemeral_shared_key::<
        PacketEncryptionAlgorithm,
        PacketHkdfAlgorithm,
        _,
    >(rng, full_address.encryption_key());

    let public_key_bytes = ephemeral_keypair.public_key().to_bytes();
    let cover_size =
        PacketSize::default().plaintext_size() - public_key_bytes.len();

    let mut cover_content: Vec<_> = DROP_COVER_MESSAGE_PAYLOAD
        .iter()
        .cloned()
        .chain(std::iter::once(1))
        .chain(std::iter::repeat(0))
        .take(cover_size)
        .collect();

    let zero_iv = stream_cipher::zero_iv::<PacketEncryptionAlgorithm>();
    stream_cipher::encrypt_in_place::<PacketEncryptionAlgorithm>(
        &shared_key,
        &zero_iv,
        &mut cover_content,
    );

    // combine it together as follows:
    // EPHEMERAL_KEY || COVER_CONTENT
    let packet_payload: Vec<_> = ephemeral_keypair.public_key().to_bytes().iter().cloned()
        .into_iter()
        .chain(cover_content.into_iter())
        .collect();

    let route =
        topology.random_route_to_gateway(rng, DEFAULT_NUM_MIX_HOPS, &full_address.gateway(), true)?;

    let delays = delays::generate_from_average_duration(route.len(), average_packet_delay);
    let destination = full_address.as_sphinx_destination();

    // once merged, that's an easy rng injection point for sphinx packets : )
    let packet = SphinxPacketBuilder::new()
        .with_payload_size(PacketSize::default().payload_size())
        .build_packet(packet_payload, &route, &destination, &delays)
        .unwrap();

    let first_hop_address =
        NymNodeRoutingAddress::try_from(route.first().unwrap().address).unwrap();

    // if client is running in vpn mode, he won't even be sending cover traffic
    Ok(MixPacket::new(first_hop_address, packet, PacketMode::Mix))
}

pub fn generate_loop_cover_packet<R>(
    rng: &mut R,
    topology: &NymTopology,
    ack_key: &AckKey,
    full_address: &Recipient,
    average_ack_delay: time::Duration,
    average_packet_delay: time::Duration,
) -> Result<MixPacket, CoverMessageError>
    where
        R: RngCore + CryptoRng,
{
    // we don't care about total ack delay - we will not be retransmitting it anyway
    let (_, ack_bytes) =
        generate_loop_cover_surb_ack(rng, topology, ack_key, full_address, average_ack_delay)?
            .prepare_for_sending();

    // cover message can't be distinguishable from a normal traffic so we have to go through
    // all the effort of key generation, encryption, etc. Note here we are generating shared key
    // with ourselves!
    let (ephemeral_keypair, shared_key) = new_ephemeral_shared_key::<
        PacketEncryptionAlgorithm,
        PacketHkdfAlgorithm,
        _,
    >(rng, full_address.encryption_key());

    let public_key_bytes = ephemeral_keypair.public_key().to_bytes();
    let cover_size =
        PacketSize::default().plaintext_size() - public_key_bytes.len() - ack_bytes.len();

    let mut cover_content: Vec<_> = LOOP_COVER_MESSAGE_PAYLOAD
        .iter()
        .cloned()
        .chain(std::iter::once(1))
        .chain(std::iter::repeat(0))
        .take(cover_size)
        .collect();

    let zero_iv = stream_cipher::zero_iv::<PacketEncryptionAlgorithm>();
    stream_cipher::encrypt_in_place::<PacketEncryptionAlgorithm>(
        &shared_key,
        &zero_iv,
        &mut cover_content,
    );

    // combine it together as follows:
    // SURB_ACK_FIRST_HOP || SURB_ACK_DATA || EPHEMERAL_KEY || COVER_CONTENT
    // (note: surb_ack_bytes contains SURB_ACK_FIRST_HOP || SURB_ACK_DATA )
    let packet_payload: Vec<_> = ack_bytes
        .into_iter()
        .chain(ephemeral_keypair.public_key().to_bytes().iter().cloned())
        .chain(cover_content.into_iter())
        .collect();

    let route =
        topology.random_route_to_gateway(rng, DEFAULT_NUM_MIX_HOPS, &full_address.gateway(), false)?;
    let delays = delays::generate_from_average_duration(route.len(), average_packet_delay);
    let destination = full_address.as_sphinx_destination();

    // once merged, that's an easy rng injection point for sphinx packets : )
    let packet = SphinxPacketBuilder::new()
        .with_payload_size(PacketSize::default().payload_size())
        .build_packet(packet_payload, &route, &destination, &delays)
        .unwrap();

    let first_hop_address =
        NymNodeRoutingAddress::try_from(route.first().unwrap().address).unwrap();

    // if client is running in vpn mode, he won't even be sending cover traffic
    Ok(MixPacket::new(first_hop_address, packet, PacketMode::Mix))
}

/// Helper function used to determine if given message represents a loop cover message.
// It kinda seems like there must exist "prefix" or "starts_with" method for bytes
// or something, but I couldn't find anything
pub fn is_cover(data: &[u8]) -> bool {
    is_cover_inner(data, &LOOP_COVER_MESSAGE_PAYLOAD) || is_cover_inner(data, &DROP_COVER_MESSAGE_PAYLOAD)
}

fn is_cover_inner(data: &[u8], drop_message: &[u8]) -> bool {
    if data.len() < drop_message.len() {
        return false;
    }

    for i in 0..drop_message.len() {
        if data[i] != drop_message[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_cover_works_for_identical_input() {
        assert!(is_cover(&LOOP_COVER_MESSAGE_PAYLOAD));
        assert!(is_cover(&DROP_COVER_MESSAGE_PAYLOAD));
    }

    #[test]
    fn is_cover_works_for_longer_input() {
        let input: Vec<_> = LOOP_COVER_MESSAGE_PAYLOAD
            .iter()
            .cloned()
            .chain(std::iter::repeat(42).take(100))
            .collect();
        assert!(is_cover(&input));

        let input: Vec<_> = DROP_COVER_MESSAGE_PAYLOAD
            .iter()
            .cloned()
            .chain(std::iter::repeat(42).take(100))
            .collect();
        assert!(is_cover(&input));
    }

    #[test]
    fn is_cover_returns_false_for_unrelated_input() {
        // make sure the length checks out
        let input: Vec<_> = LOOP_COVER_MESSAGE_PAYLOAD.iter().map(|_| 42).collect();
        assert!(!is_cover(&input));

        // make sure the length checks out
        let input: Vec<_> = DROP_COVER_MESSAGE_PAYLOAD.iter().map(|_| 42).collect();
        assert!(!is_cover(&input));
    }

    #[test]
    fn is_cover_returns_false_for_part_of_correct_input() {
        let input: Vec<_> = LOOP_COVER_MESSAGE_PAYLOAD
            .iter()
            .cloned()
            .take(LOOP_COVER_MESSAGE_PAYLOAD.len() - 1)
            .chain(std::iter::once(42))
            .collect();
        assert!(!is_cover(&input));

        let input: Vec<_> = DROP_COVER_MESSAGE_PAYLOAD
            .iter()
            .cloned()
            .take(DROP_COVER_MESSAGE_PAYLOAD.len() - 1)
            .chain(std::iter::once(42))
            .collect();
        assert!(!is_cover(&input))
    }

    #[test]
    fn is_cover_returns_false_for_empty_input() {
        let empty = Vec::new();
        assert!(!is_cover(&empty))
    }
}
