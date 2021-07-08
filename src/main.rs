use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::time::Duration;

const MULTICAST_PORT: u16 = 7373;
const MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let socket_addr: SocketAddr = std::env::var("SOCKET_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:0".to_string())
        .parse()
        .expect("invalid socket address");
    let multicast_addr: SocketAddr = std::env::var("MULTICAST_ADDRESS")
        .unwrap_or_else(|_| SocketAddr::new(MULTICAST_ADDR.into(), MULTICAST_PORT).to_string())
        .parse()
        .expect("invalid multicast address");
    let send_port: u16 = std::env::var("SEND_PORT")
        .unwrap_or_else(|_| MULTICAST_PORT.to_string())
        .parse()
        .expect("invalid send address");

    let socket = Arc::new(UdpSocket::bind(socket_addr).await?);
    if multicast_addr.is_ipv4() {
        let multicast_ip = multicast_addr.ip();
        let multicast_ip: Ipv4Addr = multicast_ip
            .to_string()
            .parse()
            .expect("invalid ipv4 multicast address");
        assert!(multicast_ip.is_multicast());
        socket
            .join_multicast_v4(multicast_ip, Ipv4Addr::new(0, 0, 0, 0))
            .expect("failed to join v4 multicast");
        socket
            .set_multicast_loop_v4(false)
            .expect("failed to unset multicast v4 loop");
    } else {
        let multicast_ip = multicast_addr.ip();
        let multicast_ip: Ipv6Addr = multicast_ip
            .to_string()
            .parse()
            .expect("invalid ipv6 multicast address");
        assert!(multicast_ip.is_multicast());
        socket
            .join_multicast_v6(&multicast_ip, 0)
            .expect("failed to join v6 multicast");
        socket
            .set_multicast_loop_v6(false)
            .expect("failed to unset multicast v6 loop");
    }
    println!("Started socket at {}", socket.local_addr()?);

    // Task to send
    let send_socket = Arc::clone(&socket);
    tokio::spawn(async move {
        let output_addr = SocketAddr::new(MULTICAST_ADDR.into(), send_port);
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            send_socket
                .send_to(b"Hello", &output_addr)
                .await
                .expect("failed to send hello");
            println!("< Sent 'Hello' to {}", &output_addr);
        }
    });

    tokio::spawn(async move {
        let mut bytes = [0u8; 1024];
        while let Ok((size, addr)) = socket.recv_from(&mut bytes).await {
            println!(
                "> Received '{}' from '{}'",
                String::from_utf8_lossy(&bytes[..size]),
                addr
            );
        }
    });

    tokio::signal::ctrl_c().await.ok();

    Ok(())
}
