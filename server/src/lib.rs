use log::{debug, error, info};
use protocol::packet::{
    DisconnectKickPayload, HandshakePayload, LoginRequestPayload, Packet, ToBytes,
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
            let mut buf = vec![0u8; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("Failed to read data from socket");

                if n == 0 {
                    return;
                }

                debug!("Received packet: {:?}", buf);

                let packet = Packet::try_from(buf.as_ref()).unwrap();

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
                            data: "2e69f1dc002ab5f7".to_string(),
                        }
                        .to_bytes()
                        .unwrap();

                        debug!("Sending handshake packet: {:?}", payload.as_ref());

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
                            level_type: "FLAT".to_string(),
                            server_mode: 1,
                            dimension: 0,
                            difficulty: 0,
                            unused_0: 0,
                            max_players: 20,
                        }
                        .to_bytes()
                        .unwrap();

                        debug!("Sending login request packet: {:?}", payload.as_ref());

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
