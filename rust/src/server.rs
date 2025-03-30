use std::error::Error;
use std::os::linux::net::SocketAddrExt;
use std::os::unix::net::{SocketAddr, UnixListener, UnixStream};

use shared::{read_data, send_data};
mod shared;

const READER_BUFFER_SIZE: usize = 1024;

fn handle(mut stream: UnixStream) {
    let data = read_data(&mut stream, READER_BUFFER_SIZE);

    println!(
        "Received from client: {:#?}",
        String::from_utf8(data.clone()).unwrap()
    );

    send_data(&mut stream, &data);
}

fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from_abstract_name(b"hidden")?;
    let listener = UnixListener::bind_addr(&addr)?;

    for stream in listener.incoming() {
        stream
            .map(|stream| {
                std::thread::spawn(|| handle(stream));
            })
            .unwrap();
    }

    Ok(())
}
