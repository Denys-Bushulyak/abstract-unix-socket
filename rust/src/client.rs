mod shared;

use std::{
    os::{
        linux::net::SocketAddrExt,
        unix::net::{SocketAddr, UnixStream},
    },
    time::UNIX_EPOCH,
};

use shared::{read_data, send_data};

const DEFAULT_BUFFER_SIZE: usize = 1024;

fn main() {
    let addr = SocketAddr::from_abstract_name(b"hidden").unwrap();
    let mut stream = UnixStream::connect_addr(&addr).unwrap();

    let message = format!(
        "date: {:?}",
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
    );

    send_data(&mut stream, message.as_bytes());

    let response = read_data(&mut stream, DEFAULT_BUFFER_SIZE);
    print!(
        "Response from server: {:#?}",
        String::from_utf8(response).unwrap()
    );
}
