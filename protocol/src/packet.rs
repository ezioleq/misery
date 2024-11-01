use std::io::{self, Cursor};

use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug, PartialEq)]
pub enum Packet {
    KeepAlive(KeepAlivePacket),
    ServerListPing(ServerListPingPacket),
    DisconnectKick(DisconnectKickPacket),
}

impl TryFrom<&[u8]> for Packet {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(value);

        let packet_id = cursor.get_u8();

        match packet_id {
            0x00 => Ok(Packet::KeepAlive(KeepAlivePacket::from_bytes(&mut cursor)?)),
            0xFE => Ok(Packet::ServerListPing(ServerListPingPacket::from_bytes(
                &mut cursor,
            )?)),
            0xFF => Ok(Packet::DisconnectKick(DisconnectKickPacket::from_bytes(
                &mut cursor,
            )?)),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown packet identifier",
            )),
        }
    }
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self>;
}

pub trait ToBytes {
    fn to_bytes(&self) -> io::Result<BytesMut>;
}

fn read_string(bytes: &mut Cursor<&[u8]>) -> io::Result<String> {
    let length = bytes.get_u16() as usize;
    let mut utf8_data = Vec::with_capacity(length);
    for _ in 0..length {
        utf8_data.push(bytes.get_u8());
    }
    String::from_utf8(utf8_data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 data"))
}

fn put_string(buffer: &mut BytesMut, s: &str) -> io::Result<()> {
    buffer.put_u16(s.len() as u16);
    for utf8_char in s.chars() {
        buffer.put_u8(utf8_char as u8);
    }
    Ok(())
}

//
// Keep alive packet
//

#[derive(Debug, PartialEq)]
pub struct KeepAlivePacket {
    keep_alive_id: i32,
}

impl FromBytes for KeepAlivePacket {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            keep_alive_id: bytes.get_i32(),
        })
    }
}

impl ToBytes for KeepAlivePacket {
    fn to_bytes(&self) -> io::Result<BytesMut> {
        let mut buffer = BytesMut::with_capacity(5);
        buffer.put_u8(0x00);
        buffer.put_i32(self.keep_alive_id);
        Ok(buffer)
    }
}

//
// Server list ping packet
//

#[derive(Debug, PartialEq)]
pub struct ServerListPingPacket;

impl FromBytes for ServerListPingPacket {
    fn from_bytes(_: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self)
    }
}

//
// Disconnect/Kick packet
//

#[derive(Debug, PartialEq)]
pub struct DisconnectKickPacket {
    pub reason: String,
}

impl FromBytes for DisconnectKickPacket {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        let reason = read_string(bytes)?;
        Ok(Self { reason })
    }
}

impl ToBytes for DisconnectKickPacket {
    fn to_bytes(&self) -> io::Result<BytesMut> {
        let mut buffer = BytesMut::with_capacity(1 + self.reason.len() * 2 + 2);
        buffer.put_u8(0xFF);
        put_string(&mut buffer, &self.reason)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_keep_alive_packet() {
        let data: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x11];

        let packet = Packet::try_from(data).unwrap();

        assert!(matches!(
            packet,
            Packet::KeepAlive(KeepAlivePacket { keep_alive_id: 17 })
        ))
    }

    #[test]
    fn encode_keep_alive_packet() {
        let packet = KeepAlivePacket { keep_alive_id: 17 };

        let data = packet.to_bytes().unwrap();

        assert_eq!(&data[..], &[0x00u8, 0x00, 0x00, 0x00, 0x11]);
    }

    #[test]
    fn decode_server_list_ping_packet() {
        let data: &[u8] = &[0xFE];

        let packet = Packet::try_from(data).unwrap();

        assert!(matches!(packet, Packet::ServerListPing { .. }))
    }

    #[test]
    fn decode_disconnect_kick_packet() {
        let data: &[u8] = &[0xFF, 0x00, 0x01, 0x41];

        let packet = Packet::try_from(data).unwrap();

        assert_eq!(
            packet,
            Packet::DisconnectKick(DisconnectKickPacket {
                reason: "A".to_string(),
            })
        );
    }

    #[test]
    fn encode_disconnect_kick_packet() {
        let packet = DisconnectKickPacket {
            reason: "A".to_string(),
        };

        let data = packet.to_bytes().unwrap();

        assert_eq!(&data[..], &[0xFFu8, 0x00, 0x01, 0x41])
    }
}
