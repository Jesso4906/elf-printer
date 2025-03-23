use elf::abi::*;
use elf::file::*;

pub fn print_file_header(bytes: &Vec<u8>) {
    if bytes[EI_CLASS] == ELFCLASS64 {
        println!("file is 64 bit.")
    } else {
        println!("file is NOT 64 bit.")
    }
}

fn print_file_header_64(bytes: &Vec<u8>) {
    println!("Magic: {:#02X}", bytes[0]);
}

fn print_file_header_32(bytes: &Vec<u8>) {
    println!("Magic: {:#02X}", bytes[0]);
}
