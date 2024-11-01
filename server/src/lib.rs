use log::{debug, info};
use protocol::packet::{DisconnectKickPacket, Packet, ToBytes};
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

                let packet = Packet::try_from(&buf[..]).unwrap();

                match packet {
                    Packet::ServerListPing(_) => {
                        debug!("Received server ping packet!");

                        let packet = DisconnectKickPacket {
                            reason: "A Minecraft Server§0§20".to_string(),
                        }
                        .to_bytes()
                        .unwrap();

                        debug!("Sending status packet: {:?}", packet.as_ref() as &[u8]);

                        socket
                            .write_all(packet.as_ref())
                            .await
                            .expect("Failed to write data to socket");
                    }
                    _ => panic!("uh nuh"),
                }
            }
        });
    }
}
