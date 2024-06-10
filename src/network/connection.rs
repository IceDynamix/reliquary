use anyhow::Context;
use etherparse::{SlicedPacket, TransportSlice, UdpHeader};
use tracing::{debug, info, instrument, trace, warn};

use crate::network::{ConnectionPacket, PacketDirection};

use anyhow::{bail, Result};

#[instrument(skip_all)]
pub fn parse_connection_packet(port_filter: &[u16], bytes: Vec<u8>) -> Result<ConnectionPacket> {
    let (udp, payload) = parse_udp(bytes)?;
    let direction = validate_ports(&port_filter, udp)?;

    if payload.len() <= 20 {
        let code = u32::from_be_bytes(
            payload
                .get(..4)
                .context("failed to read code")?
                .try_into()?,
        );
        match code {
            0xFF => {
                info!("handshake requested");
                Ok(ConnectionPacket::HandshakeRequested)
            }
            404 => {
                warn!("disconnected packet");
                Ok(ConnectionPacket::Disconnected)
            }
            _ => {
                trace!("handshake established");
                Ok(ConnectionPacket::HandshakeEstablished)
            }
        }
    } else {
        Ok(ConnectionPacket::SegmentData(direction, payload))
    }
}

#[instrument(skip_all, fields(len = data.len()))]
pub fn parse_udp(data: Vec<u8>) -> Result<(UdpHeader, Vec<u8>)> {
    let packet = SlicedPacket::from_ethernet(&data)?;

    // sanity checking the pcap filters
    let Some(transport) = packet.transport else {
        debug!("transport was not present");
        bail!("transport was not present");
    };

    let TransportSlice::Udp(udp) = transport else {
        debug!("packet was not udp");
        bail!("packet was not udp");
    };

    trace!("complete");

    Ok((udp.to_header(), udp.payload().to_vec()))
}

fn validate_ports(port_filter: &[u16], udp: UdpHeader) -> Result<PacketDirection> {
    let (src, dest) = (udp.source_port, udp.destination_port);
    if port_filter.contains(&src) {
        Ok(PacketDirection::Received)
    } else if port_filter.contains(&dest) {
        Ok(PacketDirection::Sent)
    } else {
        trace!(src, dest, "incorrect ports");
        bail!("incorrect ports");
    }
}
