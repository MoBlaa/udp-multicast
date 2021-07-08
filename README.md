# udp-multicast
Demo application for UDP multicast with Tokio UdpSocket

This application can be used to test if the machines on your network are able to receive multicasted packages.

After starting with `cargo run` or building and running `./target/udp-multicast` the application sends the message "Hello" to the multicast address `224.0.0.123:7373` every second and prints if it received packages itself.
