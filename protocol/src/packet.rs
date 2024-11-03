use std::io::{self, Cursor};

use bytes::{Buf, BufMut, BytesMut};

/// Keep alive packet identifier.
const KEEP_ALIVE_PACKET_ID: u8 = 0x00;
/// Login request identifier.
const LOGIN_REQUEST_PACKET_ID: u8 = 0x01;
/// Handshake packet identifier.
const HANDSHAKE_PACKET_ID: u8 = 0x02;
/// Chat message packet identifier.
const CHAT_MESSAGE_PACKET_ID: u8 = 0x03;
/// Time update packet identifier.
const TIME_UPDATE_PACKET_ID: u8 = 0x04;
/// Entity equipment packet identifier.
const ENTITY_EQUIPMENT_PACKET_ID: u8 = 0x05;
/// Spawn position packet identifier.
const SPAWN_POSITION_PACKET_ID: u8 = 0x06;
/// Player position and look packet identifier.
const PLAYER_POSITION_AND_LOOK_PACKET_ID: u8 = 0x0D;
/// Server list ping packet identifier.
const SERVER_LIST_PING_PACKET_ID: u8 = 0xFE;
/// Disconnect/Kick packet identifier.
const DISCONNECT_KICK_PACKET_ID: u8 = 0xFF;

/// Represents a single packet type and payload contained within it.
#[derive(Debug, PartialEq)]
pub enum Packet {
    /// Two-way, Keep Alive packet.
    KeepAlive(KeepAlivePayload),

    /// Two-way, Login request packet.
    LoginRequest(LoginRequestPayload),

    /// Two-way, Handshake packet.
    Handshake(HandshakePayload),

    /// Two-way, Chat message packet.
    ChatMessage(ChatMessagePayload),

    /// Server to Client, time update packet.
    TimeUpdate(TimeUpdatePayload),

    /// Server to Client, entity equipment packet.
    EntityEquipment(EntityEquipmentPayload),

    /// Server to Client, spawn position packet.
    SpawnPosition(SpawnPositionPayload),

    /// Two-way, Player position and look packet.
    PlayerPositionAndLook(PlayerPositionAndLookPayload),

    /// Client to Server, Server List Ping packet.
    ServerListPing(ServerListPingPayload),

    /// Two-way, Disconnect/Kick packet.
    DisconnectKick(DisconnectKickPayload),
}

impl Packet {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let mut cursor = Cursor::new(bytes);
        let packet_id = cursor.get_u8();

        match packet_id {
            KEEP_ALIVE_PACKET_ID => {
                let payload = KeepAlivePayload::from_bytes(&mut cursor)?;
                Ok(Packet::KeepAlive(payload))
            }
            LOGIN_REQUEST_PACKET_ID => {
                let payload = LoginRequestPayload::from_bytes(&mut cursor)?;
                Ok(Packet::LoginRequest(payload))
            }
            DISCONNECT_KICK_PACKET_ID => {
                let payload = DisconnectKickPayload::from_bytes(&mut cursor)?;
                Ok(Packet::DisconnectKick(payload))
            }
            HANDSHAKE_PACKET_ID => {
                let payload = HandshakePayload::from_bytes(&mut cursor)?;
                Ok(Packet::Handshake(payload))
            }
            CHAT_MESSAGE_PACKET_ID => {
                let payload = ChatMessagePayload::from_bytes(&mut cursor)?;
                Ok(Packet::ChatMessage(payload))
            }
            TIME_UPDATE_PACKET_ID => {
                let payload = TimeUpdatePayload::from_bytes(&mut cursor)?;
                Ok(Packet::TimeUpdate(payload))
            }
            ENTITY_EQUIPMENT_PACKET_ID => {
                let payload = EntityEquipmentPayload::from_bytes(&mut cursor)?;
                Ok(Packet::EntityEquipment(payload))
            }
            SPAWN_POSITION_PACKET_ID => {
                let payload = SpawnPositionPayload::from_bytes(&mut cursor)?;
                Ok(Packet::SpawnPosition(payload))
            }
            PLAYER_POSITION_AND_LOOK_PACKET_ID => {
                let payload = PlayerPositionAndLookPayload::from_bytes(&mut cursor)?;
                Ok(Packet::PlayerPositionAndLook(payload))
            }
            SERVER_LIST_PING_PACKET_ID => {
                let payload = ServerListPingPayload::from_bytes(&mut cursor)?;
                Ok(Packet::ServerListPing(payload))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unknown packet ID",
            )),
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buffer = BytesMut::new();

        match self {
            Packet::KeepAlive(payload) => {
                buffer.put_u8(KEEP_ALIVE_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
            Packet::LoginRequest(payload) => {
                buffer.put_u8(LOGIN_REQUEST_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
            Packet::Handshake(payload) => {
                buffer.put_u8(HANDSHAKE_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
            Packet::ChatMessage(payload) => {
                buffer.put_u8(CHAT_MESSAGE_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
            Packet::TimeUpdate(payload) => {
                buffer.put_u8(TIME_UPDATE_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
            Packet::EntityEquipment(payload) => {
                buffer.put_u8(ENTITY_EQUIPMENT_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
            Packet::SpawnPosition(payload) => {
                buffer.put_u8(SPAWN_POSITION_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
            Packet::PlayerPositionAndLook(payload) => {
                buffer.put_u8(PLAYER_POSITION_AND_LOOK_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
            Packet::ServerListPing(_) => {
                buffer.put_u8(SERVER_LIST_PING_PACKET_ID);
            }
            Packet::DisconnectKick(payload) => {
                buffer.put_u8(DISCONNECT_KICK_PACKET_ID);
                payload.to_bytes(&mut buffer)?;
            }
        }

        Ok(buffer.to_vec())
    }
}

/// Parse a packet payload from a byte stream.
trait FromBytes: Sized {
    /// Parses bytes to return a value of this payload.
    ///
    /// If parsing succeeds, return the value inside Ok,
    /// otherwise when the data bytes are invalid return an `io::Error`.
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self>;
}

/// Converts a packet payload to a byte buffer.
trait ToBytes {
    /// Converts a value to return a bytes representation of this payload.
    ///
    /// If converting succeeds, return the value inside Ok,
    /// otherwise when there is no more space left in the buffer return an `io::Error`.
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()>;
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
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        buffer.put_i32(self.keep_alive_id);
        Ok(())
    }
}

//
// Login request packet
//

/// Payload for the `Packet::LoginRequest`.
#[derive(Debug, PartialEq)]
pub struct LoginRequestPayload {
    /// # Client to Server
    /// The `id` is the protocol version, for 1.2.5 it should be equal to `29`.
    ///
    /// # Server to Client
    /// The `id` is the player's entity identifier.
    pub id: i32,

    /// # Client to Server
    /// Player's username.
    ///
    /// # Server to Client
    /// Not used.
    pub username: String,

    /// # Client to Server
    /// Not used, should be empty string.
    ///
    /// # Server to Client
    /// Level type defined in server properties, `default` or `FLAT`.
    pub level_type: String,

    /// # Client to Server
    /// Not used.
    ///
    /// # Server to Client
    /// Server mode, `0` for survival, `1` for creative.
    pub server_mode: i32,

    /// # Client to Server
    /// Not used.
    ///
    /// # Server to Client
    /// Dimension, `-1` for Nether, `0` for The Overworld, `1` for The End.
    pub dimension: i32,

    /// # Client to Server
    /// Not used.
    ///
    /// # Server to Client
    /// Difficulty, `0` for Peaceful, `1` for Easy, `2` for Normal, `3` for Hard.
    pub difficulty: i8,

    /// Unused, should be `0`. Previously was a world height.
    pub unused_0: u8,

    /// # Client to Server
    /// Unused.
    ///
    /// # Server to Client
    /// Max players count, used by the client to draw the player list.
    pub max_players: u8,
}

impl FromBytes for LoginRequestPayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            id: bytes.get_i32(),
            username: read_string(bytes)?,
            level_type: read_string(bytes)?,
            server_mode: bytes.get_i32(),
            dimension: bytes.get_i32(),
            difficulty: bytes.get_i8(),
            unused_0: 0,
            max_players: bytes.get_u8(),
        })
    }
}

impl ToBytes for LoginRequestPayload {
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        buffer.put_i32(self.id);
        put_string(buffer, &self.username)?;
        put_string(buffer, &self.level_type)?;
        buffer.put_i32(self.server_mode);
        buffer.put_i32(self.dimension);
        buffer.put_i8(self.difficulty);
        buffer.put_u8(self.unused_0);
        buffer.put_u8(self.max_players);
        Ok(())
    }
}

//
// Handshake packet
//

/// Payload for the `Packet::Handshake`.
#[derive(Debug, PartialEq)]
pub struct HandshakePayload {
    /// # Client to Server
    /// The `data` is username and host, for example `ezioleq;localhost:25565`.
    ///
    /// # Server to Client
    /// The `data` is a connection hash, for example `2e69f1dc002ab5f7`.
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
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        put_string(buffer, &self.data)?;
        Ok(())
    }
}

//
// Chat message
//

/// Payload for the `Packet::ChatMessage`.
#[derive(Debug, PartialEq)]
pub struct ChatMessagePayload {
    /// Content of the message.
    ///
    /// User input must be sanitized server-side.
    pub message: String,
}

impl FromBytes for ChatMessagePayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            message: read_string(bytes)?,
        })
    }
}

impl ToBytes for ChatMessagePayload {
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        put_string(buffer, &self.message)?;
        Ok(())
    }
}

//
// Time update
//

/// Payload for the `Packet::TimeUpdate`.
#[derive(Debug, PartialEq)]
pub struct TimeUpdatePayload {
    /// The world (or region) time in ticks.
    pub time: i64,
}

impl FromBytes for TimeUpdatePayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            time: bytes.get_i64(),
        })
    }
}

impl ToBytes for TimeUpdatePayload {
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        buffer.put_i64(self.time);
        Ok(())
    }
}

//
// Entity equipment
//

/// Payload for the `Packet::EntityEquipment`.
#[derive(Debug, PartialEq)]
pub struct EntityEquipmentPayload {
    /// Named entity identifier.
    entity_id: i32,

    /// Equipment slot, 0 is held, 1-4 are armor slots.
    slot: i16,

    /// Equipped item, -1 for empty slot.
    item_id: i16,

    /// Undocumented.
    damage: i16,
}

impl FromBytes for EntityEquipmentPayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            entity_id: bytes.get_i32(),
            slot: bytes.get_i16(),
            item_id: bytes.get_i16(),
            damage: bytes.get_i16(),
        })
    }
}

impl ToBytes for EntityEquipmentPayload {
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        buffer.put_i32(self.entity_id);
        buffer.put_i16(self.slot);
        buffer.put_i16(self.item_id);
        buffer.put_i16(self.damage);
        Ok(())
    }
}

//
// Spawn position packet
//

/// Payload for the `Packet::SpawnPosition`.
#[derive(Debug, PartialEq)]
pub struct SpawnPositionPayload {
    /// Spawn X in block coordinates.
    pub x: i32,
    /// Spawn Y in block coordinates.
    pub y: i32,
    /// Spawn Z in block coordinates.
    pub z: i32,
}

impl FromBytes for SpawnPositionPayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            x: bytes.get_i32(),
            y: bytes.get_i32(),
            z: bytes.get_i32(),
        })
    }
}

impl ToBytes for SpawnPositionPayload {
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        buffer.put_i32(self.x);
        buffer.put_i32(self.y);
        buffer.put_i32(self.z);
        Ok(())
    }
}

//
// Player position and look packet
//

/// Payload for the `Packet::PlayerPositionAndLook`.
#[derive(Debug, PartialEq)]
pub struct PlayerPositionAndLookPayload {
    /// Absolute X position.
    pub x: f64,

    /// # Client to Server
    /// Absolute Y position.
    ///
    /// # Server to Client
    /// Stance used to modify the player's bounding box.
    pub stance_y_0: f64,

    /// # Client to Server
    /// Stance used to modify the player's bounding box.
    ///
    /// # Server to Client
    /// Absolute Y position.
    pub stance_y_1: f64,

    /// Absolute Z position.
    pub z: f64,

    /// Absolute rotation on the X axis.
    pub yaw: f32,

    /// Absolute rotation on the Y axis.
    pub pitch: f32,

    /// Whether the client is on the ground.
    pub on_ground: u8,
}

impl FromBytes for PlayerPositionAndLookPayload {
    fn from_bytes(bytes: &mut Cursor<&[u8]>) -> io::Result<Self> {
        Ok(Self {
            x: bytes.get_f64(),
            stance_y_0: bytes.get_f64(),
            stance_y_1: bytes.get_f64(),
            z: bytes.get_f64(),
            yaw: bytes.get_f32(),
            pitch: bytes.get_f32(),
            on_ground: bytes.get_u8(),
        })
    }
}

impl ToBytes for PlayerPositionAndLookPayload {
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        buffer.put_f64(self.x);
        buffer.put_f64(self.stance_y_0);
        buffer.put_f64(self.stance_y_1);
        buffer.put_f64(self.z);
        buffer.put_f32(self.yaw);
        buffer.put_f32(self.pitch);
        buffer.put_u8(self.on_ground);
        Ok(())
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
    fn to_bytes(&self, buffer: &mut BytesMut) -> io::Result<()> {
        put_string(buffer, &self.reason)?;
        Ok(())
    }
}

// I don't know if it's a good way of unit testing, but so far it works.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_string_empty() {
        let mut buffer = BytesMut::with_capacity(2);
        put_string(&mut buffer, "").unwrap();

        assert_eq!(buffer.as_ref(), &[0x00, 0x00]);
    }

    #[test]
    fn put_string_test() {
        let mut buffer = BytesMut::with_capacity(10);
        put_string(&mut buffer, "test").unwrap();

        assert_eq!(
            buffer.as_ref(),
            &[0x00, 0x04, 0x00, 0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74]
        );
    }

    #[test]
    fn read_string_test() {
        let mut cursor =
            Cursor::new(&[0x00u8, 0x04, 0x00, 0x74, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74] as &[u8]);
        let s = read_string(&mut cursor).unwrap();

        assert_eq!(s, "test");
    }

    #[test]
    fn decode_trailing_zeroes_without_payload() {
        let data: &[u8] = &[0xFE, 0x00, 0x00, 0x00];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(packet, Packet::ServerListPing(ServerListPingPayload {}));
    }

    #[test]
    fn decode_trailing_zeroes_with_payload() {
        let data: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::KeepAlive(KeepAlivePayload { keep_alive_id: 17 })
        );
    }

    #[test]
    fn decode_keep_alive_packet() {
        let data: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x11];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::KeepAlive(KeepAlivePayload { keep_alive_id: 17 })
        );
    }

    #[test]
    fn encode_keep_alive_packet() {
        let packet = Packet::KeepAlive(KeepAlivePayload { keep_alive_id: 17 });

        let data = packet.to_bytes().unwrap();

        assert_eq!(data, &[0x00, 0x00, 0x00, 0x00, 0x11]);
    }

    #[test]
    fn decode_login_request_packet() {
        let data: &[u8] = &[
            0x01, 0x00, 0x00, 0x00, 0x1D, 0x00, 0x01, 0x00, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::LoginRequest(LoginRequestPayload {
                id: 29,
                username: "e".to_string(),
                level_type: "".to_string(),
                server_mode: 0,
                dimension: 0,
                difficulty: 0,
                unused_0: 0,
                max_players: 0
            })
        )
    }

    #[test]
    fn encode_login_request_packet() {
        let packet = Packet::LoginRequest(LoginRequestPayload {
            id: 1234,
            username: "".to_string(),
            level_type: "FLAT".to_string(),
            server_mode: 1,
            dimension: 0,
            difficulty: 0,
            unused_0: 0,
            max_players: 5,
        });

        let data = packet.to_bytes().unwrap();

        assert_eq!(
            data,
            &[
                0x01, 0x00, 0x00, 0x04, 0xD2, 0x00, 0x00, 0x00, 0x04, 0x00, 0x46, 0x00, 0x4C, 0x00,
                0x41, 0x00, 0x54, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05,
            ]
        )
    }

    #[test]
    fn decode_handshake_packet() {
        let data: &[u8] = &[0x02, 0x00, 0x03, 0x00, 0x65, 0x00, 0x3B, 0x00, 0x31];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::Handshake(HandshakePayload {
                data: "e;1".to_string()
            })
        )
    }

    #[test]
    fn encode_handshake_packet() {
        let packet = Packet::Handshake(HandshakePayload {
            data: "e;1".to_string(),
        });

        let data = packet.to_bytes().unwrap();

        assert_eq!(
            data,
            &[0x02, 0x00, 0x03, 0x00, 0x65, 0x00, 0x3B, 0x00, 0x31]
        );
    }

    #[test]
    fn decode_chat_message_packet() {
        let data: &[u8] = &[0x03, 0x00, 0x02, 0x00, b'h', 0x00, b'i'];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::ChatMessage(ChatMessagePayload {
                message: "hi".to_string()
            })
        );
    }

    #[test]
    fn encode_chat_message_packet() {
        let packet = Packet::ChatMessage(ChatMessagePayload {
            message: "hi".to_string(),
        });

        let data = packet.to_bytes().unwrap();

        assert_eq!(data, &[0x03, 0x00, 0x02, 0x00, b'h', 0x00, b'i']);
    }

    #[test]
    fn decode_time_update_packet() {
        let data: &[u8] = &[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(packet, Packet::TimeUpdate(TimeUpdatePayload { time: 16 }));
    }

    #[test]
    fn encode_time_update_packet() {
        let packet = Packet::TimeUpdate(TimeUpdatePayload { time: 16 });

        let data = packet.to_bytes().unwrap();

        assert_eq!(
            data,
            &[0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10]
        );
    }

    #[test]
    fn decode_entity_equipment_packet() {
        let data: &[u8] = &[
            0x05, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00, 0x40, 0x00, 0x00,
        ];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::EntityEquipment(EntityEquipmentPayload {
                entity_id: 32,
                slot: 4,
                item_id: 64,
                damage: 0
            })
        );
    }

    #[test]
    fn encode_entity_equipment_packet() {
        let packet = Packet::EntityEquipment(EntityEquipmentPayload {
            entity_id: 32,
            slot: 4,
            item_id: 64,
            damage: 0,
        });

        let data = packet.to_bytes().unwrap();

        assert_eq!(
            data,
            &[0x05, 0x00, 0x00, 0x00, 0x20, 0x00, 0x04, 0x00, 0x40, 0x00, 0x00]
        )
    }

    #[test]
    fn decode_spawn_position_packet() {
        let data: &[u8] = &[
            0x06, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x30,
        ];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::SpawnPosition(SpawnPositionPayload {
                x: 16,
                y: 32,
                z: 48,
            })
        );
    }

    #[test]
    fn encode_spawn_position_packet() {
        let packet = Packet::SpawnPosition(SpawnPositionPayload {
            x: 16,
            y: 32,
            z: 48,
        });

        let data = packet.to_bytes().unwrap();

        assert_eq!(
            data,
            &[0x06, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x30]
        )
    }

    #[test]
    fn decode_player_position_and_look_packet() {
        let data: &[u8] = &[
            0x0D, 64, 33, 0, 0, 0, 0, 0, 0, 64, 80, 64, 0, 0, 0, 0, 0, 64, 80, 167, 174, 20, 128,
            0, 0, 64, 33, 0, 0, 0, 0, 0, 0, 195, 52, 0, 0, 0, 0, 0, 0, 0,
        ];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::PlayerPositionAndLook(PlayerPositionAndLookPayload {
                x: 8.5,
                stance_y_0: 65.0,
                stance_y_1: 66.62000000476837,
                z: 8.5,
                yaw: -180.0,
                pitch: 0.0,
                on_ground: 0
            })
        );

        println!("{:?}", packet)
    }

    #[test]
    fn encode_player_position_and_look_packet() {
        let packet = Packet::PlayerPositionAndLook(PlayerPositionAndLookPayload {
            x: 8.5,
            stance_y_0: 65.0,
            stance_y_1: 66.62000000476837,
            z: 8.5,
            yaw: -180.0,
            pitch: 0.0,
            on_ground: 0,
        });

        let data = packet.to_bytes().unwrap();

        assert_eq!(
            data,
            &[
                0x0D, 0x40, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x50, 0x40, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x40, 0x50, 0xA7, 0xAE, 0x14, 0x80, 0x00, 0x00, 0x40, 0x21, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0xC3, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ]
        );
    }

    #[test]
    fn decode_server_list_ping_packet() {
        let data: &[u8] = &[0xFE];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(packet, Packet::ServerListPing(ServerListPingPayload {}));
    }

    #[test]
    fn decode_disconnect_kick_packet() {
        let data: &[u8] = &[0xFF, 0x00, 0x01, 0x00, b'A'];

        let packet = Packet::from_bytes(data).unwrap();

        assert_eq!(
            packet,
            Packet::DisconnectKick(DisconnectKickPayload {
                reason: "A".to_string(),
            })
        );
    }

    #[test]
    fn encode_disconnect_kick_packet() {
        let packet = Packet::DisconnectKick(DisconnectKickPayload {
            reason: "A".to_string(),
        });

        let data = packet.to_bytes().unwrap();

        assert_eq!(data, &[0xFF, 0x00, 0x01, 0x00, b'A'])
    }

    #[test]
    fn encode_disconnect_server_status_packet() {
        let expected_data = &[
            0xFF, 0x00, 0x08, 0x00, 0x45, 0x00, 0x5A, 0x00, 0x49, 0x00, 0x4F, 0x00, 0xA7, 0x00,
            0x34, 0x00, 0xA7, 0x00, 0x34,
        ];

        let packet = Packet::DisconnectKick(DisconnectKickPayload {
            reason: "EZIO§4§4".to_string(),
        });

        let data = packet.to_bytes().unwrap();

        assert_eq!(data, expected_data)
    }
}
