use log::{debug, error, info, trace};
use protocol::packet::{
    DisconnectKickPayload, HandshakePayload, LoginRequestPayload, Packet,
    PlayerPositionAndLookPayload, SpawnPositionPayload,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn send_packet(socket: &mut TcpStream, packet: Packet) -> std::io::Result<()> {
    let data = packet.to_bytes()?;
    socket.write_all(&data).await?;

    trace!("Sent: {:?} {:02X?}", packet, &data);
    Ok(())
}

pub async fn start_server() {
    info!("Hello! :3");
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        debug!("Connection from {:?}", &addr);

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 256];

            loop {
                let n = socket
                    .read(&mut buffer)
                    .await
                    .expect("Failed to read data from socket");

                if n == 0 {
                    return;
                }

                trace!("Received: {:02X?}", buffer);

                let Ok(packet) = Packet::from_bytes(buffer.as_ref()) else {
                    return;
                };

                match packet {
                    Packet::ServerListPing(_) => {
                        debug!("Received server ping packet!");

                        send_packet(
                            &mut socket,
                            Packet::DisconnectKick(DisconnectKickPayload {
                                reason: "A Minecraft Server§0§20".to_string(),
                            }),
                        )
                        .await
                        .unwrap();
                    }
                    Packet::Handshake(_) => {
                        debug!("Received handshake packet!");

                        send_packet(
                            &mut socket,
                            Packet::Handshake(HandshakePayload {
                                data: "-".to_string(),
                            }),
                        )
                        .await
                        .unwrap();
                    }
                    Packet::LoginRequest(_) => {
                        debug!("Received login request packet!");

                        send_packet(
                            &mut socket,
                            Packet::LoginRequest(LoginRequestPayload {
                                id: 1234,
                                username: "".to_string(),
                                level_type: "default".to_string(),
                                server_mode: 1,
                                dimension: 0,
                                difficulty: 0,
                                unused_0: 0,
                                max_players: 20,
                            }),
                        )
                        .await
                        .unwrap();

                        // spawn position

                        send_packet(
                            &mut socket,
                            Packet::SpawnPosition(SpawnPositionPayload { x: 8, y: 65, z: 8 }),
                        )
                        .await
                        .unwrap();

                        // position and look

                        send_packet(
                            &mut socket,
                            Packet::PlayerPositionAndLook(PlayerPositionAndLookPayload {
                                x: 8.5,
                                stance_y_0: 66.62,
                                stance_y_1: 65.0,
                                z: 8.5,
                                yaw: -180.0,
                                pitch: 0.0,
                                on_ground: 0,
                            }),
                        )
                        .await
                        .unwrap();
                    }
                    Packet::PlayerPositionAndLook(position_and_look) => {
                        debug!("Received player position and look packet!",);

                        send_packet(
                            &mut socket,
                            Packet::PlayerPositionAndLook(PlayerPositionAndLookPayload {
                                x: position_and_look.x,
                                stance_y_0: position_and_look.stance_y_0,
                                stance_y_1: position_and_look.stance_y_1,
                                z: position_and_look.z,
                                yaw: position_and_look.yaw,
                                pitch: position_and_look.pitch,
                                on_ground: position_and_look.on_ground,
                            }),
                        )
                        .await
                        .unwrap();
                    }
                    _ => error!("Unhandled packet type"),
                }
            }
        });
    }
}
