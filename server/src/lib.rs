use log::{debug, error, info, trace};
use protocol::packet::{
    DisconnectKickPayload, HandshakePayload, KeepAlivePayload, LoginRequestPayload, Packet,
    PlayerPositionAndLookPayload, SpawnPositionPayload, ToBytes,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub async fn start_server() {
    info!("Hello! :3");
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        debug!("Connection from {:?}", &addr);

        tokio::spawn(async move {
            let mut buf = vec![0u8; 128];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("Failed to read data from socket");

                if n == 0 {
                    return;
                }

                trace!("Received packet: {:?}", buf);

                let Ok(packet) = Packet::try_from(buf.as_ref()) else {
                    return;
                };

                match packet {
                    Packet::ServerListPing(_) => {
                        debug!("Received server ping packet!");

                        let payload = DisconnectKickPayload {
                            reason: "A Minecraft Server§0§20".to_string(),
                        }
                        .to_bytes()
                        .unwrap();

                        debug!("Sending status packet: {:?}", payload.as_ref());

                        socket
                            .write_all(payload.as_ref())
                            .await
                            .expect("Failed to write data to socket");
                    }
                    Packet::Handshake(handshake) => {
                        debug!("Received handshake packet! {:?}", handshake);

                        let payload = HandshakePayload {
                            data: "-".to_string(),
                        }
                        .to_bytes()
                        .unwrap();

                        trace!("Sending handshake packet: {:?}", payload.as_ref());

                        socket
                            .write_all(payload.as_ref())
                            .await
                            .expect("Failed to write data to socket");
                    }
                    Packet::LoginRequest(login_request) => {
                        debug!("Received login request packet! {:?}", login_request);

                        let payload = LoginRequestPayload {
                            id: 1234,
                            username: "".to_string(),
                            level_type: "default".to_string(),
                            server_mode: 1,
                            dimension: 0,
                            difficulty: 0,
                            unused_0: 0,
                            max_players: 20,
                        }
                        .to_bytes()
                        .unwrap();

                        trace!("Sending login request packet: {:?}", payload.as_ref());

                        socket
                            .write_all(payload.as_ref())
                            .await
                            .expect("Failed to write data to socket");

                        // spawn position

                        let payload = SpawnPositionPayload { x: 8, y: 65, z: 8 }
                            .to_bytes()
                            .unwrap();

                        trace!("Sending spawn position packet: {:?}", payload.as_ref());

                        socket
                            .write_all(payload.as_ref())
                            .await
                            .expect("Failed to write data to socket");

                        // position and look

                        let payload = PlayerPositionAndLookPayload {
                            x: 8.5,
                            stance_y_0: 66.62,
                            stance_y_1: 65.0,
                            z: 8.5,
                            yaw: -180.0,
                            pitch: 0.0,
                            on_ground: 0,
                        }
                        .to_bytes()
                        .unwrap();

                        trace!(
                            "Sending player position and look packet: {:?}",
                            payload.as_ref()
                        );

                        socket
                            .write_all(payload.as_ref())
                            .await
                            .expect("Failed to write data to socket");
                    }
                    Packet::PlayerPositionAndLook(position_and_look) => {
                        debug!(
                            "Received player position and look packet! {:?}",
                            position_and_look
                        );

                        let payload = PlayerPositionAndLookPayload {
                            x: position_and_look.x,
                            stance_y_0: position_and_look.stance_y_0,
                            stance_y_1: position_and_look.stance_y_1,
                            z: position_and_look.z,
                            yaw: position_and_look.yaw,
                            pitch: position_and_look.pitch,
                            on_ground: position_and_look.on_ground,
                        }
                        .to_bytes()
                        .unwrap();

                        debug!(
                            "Sending player position and look packet: {:?}",
                            payload.as_ref()
                        );

                        let payload = KeepAlivePayload { keep_alive_id: 1 }.to_bytes().unwrap();

                        trace!("Sending keep alive packet: {:?}", payload.as_ref());

                        socket
                            .write_all(payload.as_ref())
                            .await
                            .expect("Failed to write data to socket");
                    }
                    _ => error!("Unhandled packet type"),
                }
            }
        });
    }
}
