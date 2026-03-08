use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str::from_utf8,
};

pub fn create_write_file() {
    let mut file = match File::create("mytext.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    // Writes some prefix of the byte string, not necessarily all of it.
    let n = file
        .write(b"hello world. this is rust language and we are writing code")
        .expect("couldn't write");
    println!("{}", n);
}

pub fn create_write_all_file() {
    let mut file = match File::create("mytext2.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    // it will write all. You don’t need to worry about partial writes.
    file.write_all(b"hello world").expect("couldn't write");
}

pub fn create_bufwriter_file() {
    let file = match File::create("mytext2.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    writer
        .write_all(b"hello world. this is file2")
        .expect("couldn't write");

    writer.flush().unwrap();
}

pub fn create_bufwriter_writer_file() {
    let file = match File::create("mytext2.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    let mut text_bytes: &[u8] =
        b"hello world. this is file2. and we are using bufwriter with writer fn";
    while !text_bytes.is_empty() {
        // Writes some prefix of the byte string, not necessarily all of it.
        let n = writer.write(text_bytes).expect("couldn't write");
        text_bytes = &text_bytes[n..];
    }

    writer.flush().unwrap();
}

pub fn read_file() {
    let mut buffer = [0; 20];

    let mut file = match File::open("mytext.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let n = file.read(&mut buffer).expect("couldn't read");
    println!("{:?}", &buffer);
    println!("{}", n);

    let to_text = from_utf8(&buffer[..n]).expect("invalid utf8");
    println!("{}", to_text);
}

pub fn read_file_tostr() {
    let mut buffer = String::new();

    let mut file = match File::open("mytext.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    file.read_to_string(&mut buffer).expect("couldn't read");
    println!("{}", buffer)

    /*
     * Triggers more system calls
     * Switches from your program (user space) → into the OS (kernel space)
     * The OS fetches bytes and Returns them back to your program
     * That user < -- > kernel switch is expensive compared to normal memory access.
     */
}

pub fn read_file_bufreader_tostr() {
    let mut buffer = String::new();

    let file = match File::open("mytext.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    let mut buf_reader = BufReader::new(file);

    buf_reader
        .read_to_string(&mut buffer)
        .expect("couldn't read");
    println!("{}", buffer)

    /*
     * Triggers less system calls
     * adds a memory buffer in user space
     *
     * OS read -> 8KB chunk -> store in buffer (makes syscall)
     * Serve small reads from memory (no syscall)
     * Serve small reads from memory (no syscall)
     * Serve small reads from memory (no syscall)
     * When empty -> OS read another 8KB (makes syscall)
     *
     * better performance
     */
}

pub fn read_file_stream() {
    let mut buffer = [0; 10];

    let mut file = match File::open("mytext.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    // there is a cursor that moves forward after every 10(buffer) bytes
    loop {
        let n = file.read(&mut buffer).expect("couldn't read");
        if n == 0 {
            break;
        }

        // let to_text = from_utf8(&buffer[..n]).expect("invalid utf8");
        let to_text = String::from_utf8_lossy(&buffer[..n]);
        println!("{}", to_text);
    }
}

pub fn custom_buffer_bufferreader() {
    let file = File::open("mytext2.txt").expect("unable to open");

    // 2KB buffer (2 * 1024)
    let mut reader = BufReader::with_capacity(2 * 1024, file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("unable to read");

    let text = String::from_utf8_lossy(&buffer);
    println!("{:?}", text);
}
