use std::io::{self, Cursor};

use bytes::{Buf, BufMut, BytesMut};

/// Keep alive packet identifier.
const KEEP_ALIVE_PACKET_ID: u8 = 0x00;
/// Handshake packet identifier.
const HANDSHAKE_PACKET_ID: u8 = 0x02;
/// Server list ping packet identifier.
const SERVER_LIST_PING_PACKET_ID: u8 = 0xFE;
/// Disconnect/Kick packet identifier.
const DISCONNECT_KICK_PACKET_ID: u8 = 0xFF;

/// Represents a single packet type and payload contained within it.
#[derive(Debug, PartialEq)]
pub enum Packet {
    /// Two-way, Keep Alive packet.
    KeepAlive(KeepAlivePayload),

    /// Two-way, Handshake packet.
    Handshake(HandshakePayload),

    /// Client to Server, Server List Ping packet.
    ServerListPing(ServerListPingPayload),

    /// Two-way, Disconnect/Kick packet.
    DisconnectKick(DisconnectKickPayload),
}

impl TryFrom<&[u8]> for Packet {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(value);

        let packet_id = cursor.get_u8();

        match packet_id {
            KEEP_ALIVE_PACKET_ID => Ok(Packet::KeepAlive(KeepAlivePayload::from_bytes(
                &mut cursor,
            )?)),
            HANDSHAKE_PACKET_ID => Ok(Packet::Handshake(HandshakePayload::from_bytes(
                &mut cursor,
            )?)),
            SERVER_LIST_PING_PACKET_ID => Ok(Packet::ServerListPing(
                ServerListPingPayload::from_bytes(&mut cursor)?,
            )),
            DISCONNECT_KICK_PACKET_ID => Ok(Packet::DisconnectKick(
                DisconnectKickPayload::from_bytes(&mut cursor)?,
            )),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown packet identifier",
            )),
        }
    }
}

/// Parse a packet from a byte stream.
pub trait FromBytes: Sized {
    /// Parses bytes to return a value of this packet.
    ///
    /// If parsing succeeds, return the value inside Ok,
    /// otherwise when the data bytes are invalid return an `io::Error`.
    ///
    /// ## Example
    /// ```
    /// use std::io::Cursor;
    /// use protocol::packet::{KeepAlivePayload, FromBytes};
    ///
    /// let mut cursor = Cursor::new(&[0x0u8, 0x0, 0x0, 0x0, 0x0] as &[u8]);
    /// let payload = KeepAlivePayload::from_bytes(&mut cursor);
    /// ```
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self>;
}

/// Converts a packet to a byte buffer.
pub trait ToBytes {
    /// Converts a value to return a bytes representation of this packet.
    /// Encoded packet contains its appropriate identifier as a first byte.
    ///
    /// If converting succeeds, return the value inside Ok,
    /// otherwise when there is no more space left in the buffer return an `io::Error`.
    ///
    /// ## Example
    /// ```
    /// use protocol::packet::{KeepAlivePayload, ToBytes};
    ///
    /// let payload = KeepAlivePayload { keep_alive_id: 3 };
    /// let bytes = payload.to_bytes().unwrap();
    /// ```
    fn to_bytes(&self) -> io::Result<BytesMut>;
}

/// Reads a UTF-16 encoded string from a byte stream.
///
/// Reads a `u16` length prefix at first, followed by that many `u16`
/// elements, then converts them to a `String`.
///
/// The number of elements refers to the number of characters, not the number of bytes.
fn read_string(bytes: &mut Cursor<&[u8]>) -> io::Result<String> {
    let length = bytes.get_u16() as usize;
    let mut utf16_data = Vec::with_capacity(length);

    for _ in 0..length {
        utf16_data.push(bytes.get_u16());
    }

    String::from_utf16(&utf16_data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-16 data"))
}

/// Puts a UTF-16 encoded string to a byte buffer.
///
/// Puts a `u16` length prefix at the beginning, followed by that many `u16`
/// encoded characters.
///
/// The length refers to the number of characters, not the number of bytes.
fn put_string(buffer: &mut BytesMut, s: &str) -> io::Result<()> {
    let utf16_data: Vec<u16> = s.encode_utf16().collect();
    buffer.put_u16(s.chars().count() as u16);

    for utf16_char in utf16_data {
        buffer.put_u16(utf16_char);
    }

    Ok(())
}

//
// Keep alive packet
//

/// Payload for the `Packet::KeepAlive`.
#[derive(Debug, PartialEq)]
pub struct KeepAlivePayload {
    /// Server-generated random identifier.
    pub keep_alive_id: i32,
}

impl FromBytes for KeepAlivePayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            keep_alive_id: bytes.get_i32(),
        })
    }
}

impl ToBytes for KeepAlivePayload {
    fn to_bytes(&self) -> io::Result<BytesMut> {
        let mut buffer = BytesMut::with_capacity(5);
        buffer.put_u8(KEEP_ALIVE_PACKET_ID);
        buffer.put_i32(self.keep_alive_id);
        Ok(buffer)
    }
}

//
// Handshake packet
//

#[derive(Debug, PartialEq)]
pub struct HandshakePayload {
    /// # Client to Server
    /// The data is username and host, for example `ezioleq;localhost:25565`.
    ///
    /// # Server to Client
    /// The data is a connection hash, for example `2e69f1dc002ab5f7`.
    pub data: String,
}

impl FromBytes for HandshakePayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            data: read_string(bytes)?,
        })
    }
}

impl ToBytes for HandshakePayload {
    fn to_bytes(&self) -> io::Result<BytesMut> {
        let mut buffer = BytesMut::with_capacity(3 + self.data.chars().count() * 2);
        buffer.put_u8(HANDSHAKE_PACKET_ID);
        put_string(&mut buffer, &self.data)?;
        Ok(buffer)
    }
}

//
// Server list ping packet
//

/// Payload for the `Packet::ServerListPing`.
#[derive(Debug, PartialEq)]
pub struct ServerListPingPayload;

impl FromBytes for ServerListPingPayload {
    fn from_bytes(_: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self)
    }
}

//
// Disconnect/Kick packet
//

/// Payload for the `Packet::DisconnectKick`.
#[derive(Debug, PartialEq)]
pub struct DisconnectKickPayload {
    /// Reason displayed to the client when the connection terminates.
    pub reason: String,
}

impl FromBytes for DisconnectKickPayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        let reason = read_string(bytes)?;
        Ok(Self { reason })
    }
}

impl ToBytes for DisconnectKickPayload {
    fn to_bytes(&self) -> io::Result<BytesMut> {
        let mut buffer = BytesMut::with_capacity(1 + 2 + self.reason.chars().count() * 2);
        buffer.put_u8(DISCONNECT_KICK_PACKET_ID);
        put_string(&mut buffer, &self.reason)?;
        Ok(buffer)
    }
}

// I don't know if it's a good way of unit testing, but so far it works.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_trailing_zeroes_without_payload() {
        let data: &[u8] = &[SERVER_LIST_PING_PACKET_ID, 0x00, 0x00, 0x00];

        let packet = Packet::try_from(data).unwrap();

        assert!(matches!(packet, Packet::ServerListPing { .. }))
    }

    #[test]
    fn decode_trailing_zeroes_with_payload() {
        let data: &[u8] = &[
            KEEP_ALIVE_PACKET_ID,
            0x00,
            0x00,
            0x00,
            0x11,
            0x00,
            0x00,
            0x00,
            0x00,
        ];

        let packet = Packet::try_from(data).unwrap();

        assert!(matches!(
            packet,
            Packet::KeepAlive(KeepAlivePayload { keep_alive_id: 17 })
        ))
    }

    #[test]
    fn decode_keep_alive_packet() {
        let data: &[u8] = &[KEEP_ALIVE_PACKET_ID, 0x00, 0x00, 0x00, 0x11];

        let packet = Packet::try_from(data).unwrap();

        assert!(matches!(
            packet,
            Packet::KeepAlive(KeepAlivePayload { keep_alive_id: 17 })
        ))
    }

    #[test]
    fn encode_keep_alive_packet() {
        let packet = KeepAlivePayload { keep_alive_id: 17 };

        let data = packet.to_bytes().unwrap();

        assert_eq!(&data[..], &[KEEP_ALIVE_PACKET_ID, 0x00, 0x00, 0x00, 0x11]);
    }

    #[test]
    fn decode_handshake_packet() {
        let data: &[u8] = &[
            HANDSHAKE_PACKET_ID,
            0x00,
            0x03,
            0x00,
            0x65,
            0x00,
            0x3B,
            0x00,
            0x31,
        ];

        let packet = Packet::try_from(data).unwrap();

        assert_eq!(
            packet,
            Packet::Handshake(HandshakePayload {
                data: "e;1".to_string()
            })
        )
    }

    #[test]
    fn encode_handshake_packet() {
        let packet = HandshakePayload {
            data: "e;1".to_string(),
        };

        let data = packet.to_bytes().unwrap();

        assert_eq!(
            data.as_ref(),
            &[
                HANDSHAKE_PACKET_ID,
                0x00,
                0x03,
                0x00,
                0x65,
                0x00,
                0x3B,
                0x00,
                0x31
            ]
        );
    }

    #[test]
    fn decode_server_list_ping_packet() {
        let data: &[u8] = &[SERVER_LIST_PING_PACKET_ID];

        let packet = Packet::try_from(data).unwrap();

        assert!(matches!(packet, Packet::ServerListPing { .. }))
    }

    #[test]
    fn decode_disconnect_kick_packet() {
        let data: &[u8] = &[DISCONNECT_KICK_PACKET_ID, 0x00, 0x01, 0x00, b'A'];

        let packet = Packet::try_from(data).unwrap();

        assert_eq!(
            packet,
            Packet::DisconnectKick(DisconnectKickPayload {
                reason: "A".to_string(),
            })
        );
    }

    #[test]
    fn encode_disconnect_kick_packet() {
        let packet = DisconnectKickPayload {
            reason: "A".to_string(),
        };

        let data = packet.to_bytes().unwrap();

        assert_eq!(
            &data[..],
            &[DISCONNECT_KICK_PACKET_ID, 0x00, 0x01, 0x00, b'A']
        )
    }

    #[test]
    fn encode_disconnect_server_status_packet() {
        let expected_data = &[
            DISCONNECT_KICK_PACKET_ID,
            0x00,
            0x08, // string length
            0x00,
            0x45, // E
            0x00,
            0x5A, // Z
            0x00,
            0x49, // I
            0x00,
            0x4F, // O
            0x00,
            0xA7, // §
            0x00,
            0x34, // 4
            0x00,
            0xA7, // §
            0x00,
            0x34, // 4
        ];

        let packet = DisconnectKickPayload {
            reason: "EZIO§4§4".to_string(),
        };

        let data = packet.to_bytes().unwrap();

        assert_eq!(&data[..], expected_data)
    }
}
