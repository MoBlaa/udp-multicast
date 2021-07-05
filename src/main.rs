use tokio::net::UdpSocket;
use std::net::{SocketAddr, Ipv4Addr};
use std::error::Error;
use std::sync::Arc;
use tokio::time::Duration;

const MULTICAST_PORT: u16 = 7373;
const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket = Arc::new(UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], MULTICAST_PORT))).await?);
    socket.join_multicast_v4(MULTICAST_ADDR, Ipv4Addr::new(0, 0, 0, 0)).unwrap();
    socket.set_multicast_loop_v4(true).unwrap();

    let multicast_addr = SocketAddr::new(MULTICAST_ADDR.into(), MULTICAST_PORT);

    // Task to send
    let send_socket = Arc::clone(&socket);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            send_socket.send_to(b"Hello", &multicast_addr).await.unwrap();
            println!("< Sent 'Hello'");
        }
    });

    tokio::spawn(async move {
        let mut bytes = [0u8; 1024];
        while let Ok((size, addr)) = socket.recv_from(&mut bytes).await {
            println!("> Received '{}' from '{}'", String::from_utf8_lossy(&bytes[..size]), addr);
        }
    });

    tokio::signal::ctrl_c().await.ok();

    Ok(())
}
