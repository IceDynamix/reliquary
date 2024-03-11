use etherparse::{SlicedPacket, TransportSlice, UdpHeader};
use tracing::{debug, info, Level, span, trace, warn};

use crate::sniffer::PacketDirection;

pub enum ConnectionPacket {
    HandshakeRequested,
    Disconnected,
    HandshakeEstablished,
    SegmentData(PacketDirection, Vec<u8>),
}

pub(crate) fn parse_connection_packet(port_filter: &[u16], bytes: Vec<u8>) -> Option<ConnectionPacket> {
    let (udp, payload) = parse_udp(bytes)?;
    let direction = validate_ports(&port_filter, udp)?;

    if payload.len() <= 20 {
        let code = u32::from_be_bytes(payload[..4].try_into().unwrap());
        match code {
            0xFF => {
                info!("handshake requested");
                Some(ConnectionPacket::HandshakeRequested)
            }
            404 => {
                warn!("disconnected packet");
                Some(ConnectionPacket::Disconnected)
            }
            _ => {
                trace!("handshake established");
                Some(ConnectionPacket::HandshakeEstablished)
            }
        }
    } else {
        Some(ConnectionPacket::SegmentData(direction, payload))
    }
}

pub(crate) fn parse_udp(data: Vec<u8>) -> Option<(UdpHeader, Vec<u8>)> {
    let span = span!(Level::INFO, "processing");
    let _enter = span.enter();

    let packet = match SlicedPacket::from_ethernet(&data) {
        Ok(p) => p,
        Err(e) => {
            debug!("processing failed: {e}");
            return None;
        }
    };

    // sanity checking the pcap filters
    let Some(transport) = packet.transport else {
        debug!("transport was not present");
        return None;
    };

    let TransportSlice::Udp(udp) = transport else {
        debug!("packet was not udp");
        return None;
    };

    trace!("process complete");

    Some((udp.to_header(), udp.payload().to_vec()))
}

fn validate_ports(port_filter: &[u16], udp: UdpHeader) -> Option<PacketDirection> {
    let (src, dest) = (udp.source_port, udp.destination_port);
    if port_filter.contains(&src) {
        Some(PacketDirection::Received)
    } else if port_filter.contains(&dest) {
        Some(PacketDirection::Sent)
    } else {
        trace!(src, dest, "incorrect ports");
        return None;
    }
}
