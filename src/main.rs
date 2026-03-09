use std::fs::File;

use crate::std_rs::{
    fs_file::custom_buffer_bufferwriter,
    fs_file_traits::{copy_all, file_seek, read_first_byte},
};

mod std_rs;

fn main() {
    let mut open_file = File::open("mytext3.txt").expect("unable to open");
    file_seek(&mut open_file, 6);
}
