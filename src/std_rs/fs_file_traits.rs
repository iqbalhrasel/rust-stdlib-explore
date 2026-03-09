use std::io::{Read, Write};

pub fn read_first_byte<T: Read>(reader: &mut T) {
    let mut buffer = [0; 8];
    reader.read(&mut buffer).expect("unable to read");
    println!("{:?}", buffer);
}

pub fn copy_all<R: Read, W: Write>(reader: &mut R, writer: &mut W) {
    let mut buffer = [0; 1024];

    loop {
        let n = reader.read(&mut buffer).expect("unable to read");
        if n == 0 {
            break;
        }
        writer.write_all(&buffer[..n]).expect("unable to write");
    }
}
