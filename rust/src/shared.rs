use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

const END_OF_DATA_MARKER: &[u8; 1] = b"\0";
const END_OF_DATA_SIZE: usize = size_of_val(END_OF_DATA_MARKER);

pub fn send_data(stream: &mut UnixStream, data: &[u8]) {
    let data = create_data_packet(data);

    let _ = stream.write_all(&data);
    stream.flush().unwrap();
}

pub fn read_data(stream: &mut UnixStream, buffer_size: usize) -> Vec<u8> {
    let mut data = vec![];
    let mut buf = vec![0; buffer_size];

    loop {
        match stream.read(&mut buf) {
            Ok(result) if result > 0 => {
                data.extend(&buf[..result]);

                if Some(END_OF_DATA_MARKER) == data.last_chunk::<END_OF_DATA_SIZE>() {
                    data = remove_special_symbol(&data).to_vec();
                    break;
                }
            }
            _ => break,
        }
    }

    data
}

fn remove_special_symbol(data: &[u8]) -> &[u8] {
    let end = size_of_val(data) - END_OF_DATA_SIZE;
    &data[0..end]
}

pub fn create_data_packet(data: &[u8]) -> Vec<u8> {
    [data, END_OF_DATA_MARKER].concat()
}
