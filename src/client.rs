use std::{
    io::Write,
    os::{
        linux::net::SocketAddrExt,
        unix::net::{SocketAddr, UnixStream},
    },
};

fn main() {
    let addr = SocketAddr::from_abstract_name(b"hidden").unwrap();
    let mut stream = UnixStream::connect_addr(&addr).unwrap();
    let result = stream.write(b"Hello, world");
    dbg!(result);
}
