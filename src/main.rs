use std::fs::File;

use crate::std_rs::{
    fs_file::custom_buffer_bufferwriter,
    fs_file_traits::{copy_all, read_first_byte},
};

mod std_rs;

fn main() {
    let mut open_file = File::open("mytext3.txt").expect("unable to open");
    let mut create_file = File::create("mytext4.txt").expect("unable to write");
    copy_all(&mut open_file, &mut create_file);
}
