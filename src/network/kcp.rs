use std::time::SystemTime;

use kcp::{get_conv, Kcp, KCP_OVERHEAD};
use thiserror::Error;
use tracing::{error, info, instrument, span, trace, warn, Level};

use crate::network::bytes_as_hex;

#[derive(Error, Debug)]
pub enum KcpError {
    #[error("kcp header must be at least {expected} bytes, but was {actual}")]
    HeaderTooShort { expected: usize, actual: usize },
    #[error("kcp client was not constructed")]
    ClientNotConstructed,
    #[error(
        "kcp packet does not belong to expected conversation (expected {expected}, was {actual})"
    )]
    PacketDoesNotBelongToConversation { expected: u32, actual: u32 },
    #[error(transparent)]
    InnerKcpError(#[from] kcp::Error),
}

pub(crate) struct KcpSniffer {
    conv_id: u32,
    kcp: Kcp<Vec<u8>>,
    time_start: SystemTime,
}

impl KcpSniffer {
    #[instrument(skip(segment))]
    pub fn try_new(segment: &[u8]) -> Result<KcpSniffer, KcpError> {
        validate_kcp_segment(segment).map(Self::new)
    }

    #[instrument]
    fn new(conv_id: u32) -> Self {
        info!("new connection, created new kcp instance");

        KcpSniffer {
            conv_id,
            kcp: new_kcp(conv_id),
            time_start: SystemTime::now(),
        }
    }

    #[instrument(skip_all, fields(conv_id = self.conv_id, len = segments.len()))]
    pub fn receive_segments(&mut self, segments: &[u8]) -> Result<Vec<Vec<u8>>, KcpError> {
        let conv_id = validate_kcp_segment(segments)?;

        trace!("message data: {}", bytes_as_hex(segments));

        if conv_id != self.conv_id {
            warn!(
                expected = self.conv_id,
                "packet did not belong to conversation"
            );
            return Err(KcpError::PacketDoesNotBelongToConversation {
                expected: self.conv_id,
                actual: conv_id,
            });
        }

        // game uses special format which adds 4 bytes at index 4,
        // reprocess to discard bytes 4..8 of every segment
        let segments = reformat_kcp_segments(segments);

        self.kcp.input(&segments)?;

        let mut recv = Vec::new();
        while let Ok(size) = self.kcp.peeksize() {
            let span = span!(Level::TRACE, "receiving", size);
            let _enter = span.enter();

            let mut bytes = vec![0; size];

            match self.kcp.recv(&mut bytes) {
                Ok(_size) => {
                    recv.push(bytes);
                }
                Err(e) => {
                    warn!(%e, "could not receive kcp bytes");
                    // error ignored
                }
            }
        }

        let _ = self.kcp.update(self.clock()); // error ignored

        Ok(recv)
    }

    #[inline]
    fn clock(&self) -> u32 {
        SystemTime::now()
            .duration_since(self.time_start)
            .expect("time went backwards")
            .as_millis() as u32
    }
}

#[inline]
fn new_kcp(conv_id: u32) -> Kcp<Vec<u8>> {
    let mut kcp = Kcp::new(conv_id, Vec::new());
    kcp.set_wndsize(1024, 1024);
    kcp
}

fn validate_kcp_segment(payload: &[u8]) -> Result<u32, KcpError> {
    if payload.len() <= KCP_OVERHEAD {
        warn!(
            len = payload.len(),
            data = bytes_as_hex(payload),
            "kcp header was too short"
        );
        return Err(KcpError::HeaderTooShort {
            expected: KCP_OVERHEAD,
            actual: payload.len(),
        });
    }
    Ok(get_conv(payload))
}

// reformat to skip bytes 4..8
fn reformat_kcp_segments(data: &[u8]) -> Vec<u8> {
    let span = span!(Level::TRACE, "split");
    let _enter = span.enter();

    let mut reformatted_bytes = Vec::new();

    trace!("before split: {}", bytes_as_hex(data));

    let mut i = 0;
    while i < data.len() {
        let conv_id = &data[i..i + 4];

        let _ = &data[i + 4..i + 8]; // skip

        let remaining_header = &data[i + 8..i + 28];
        let content_len = u32::from_le_bytes(data[i + 24..i + 28].try_into().unwrap()) as usize;
        let content = &data[i + 28..i + 28 + content_len];

        for b in conv_id.iter().chain(remaining_header).chain(content) {
            reformatted_bytes.push(*b);
        }

        i += 28 + content_len;
    }

    trace!(" after split: {}", bytes_as_hex(&reformatted_bytes));

    reformatted_bytes
}
