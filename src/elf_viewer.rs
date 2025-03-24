mod value_meanings;
use elf::abi::*;
use elf::file::*;

pub fn check_magic(bytes: &Vec<u8>) -> bool {
    return bytes.len() >= 4 && bytes[0] == 0x7F && bytes[1] == b'E' && bytes[2] == b'L' && bytes[3] == b'F';
}

pub fn print_file_header(bytes: &Vec<u8>) {
    if !check_magic(bytes) {
        println!("Invalid ELF binary.");
        return;
    }

    if bytes.len() >= size_of::<Elf64_Ehdr>() && bytes[EI_CLASS] == ELFCLASS64 {
        let header: Elf64_Ehdr = unsafe { std::ptr::read(bytes.as_ptr() as *const Elf64_Ehdr) };
        print_file_header_64(&header);
    } else if bytes.len() >= size_of::<Elf32_Ehdr>() && bytes[EI_CLASS] == ELFCLASS32 {
        let header: Elf32_Ehdr = unsafe { std::ptr::read(bytes.as_ptr() as *const Elf32_Ehdr) };
        print_file_header_32(&header);
    }
    else {
        println!("File has unknown architecture or bytes buffer is too small.");
    }
}

fn print_file_header_64(header: &Elf64_Ehdr) {
    println!("\nMagic (e_ident[0..4]): 0x7F ELF"); // assuming the header is valid
    println!("Architecture (e_ident[EI_CLASS]): {} ({:#04X})", value_meanings::get_ei_class_meaning(header.e_ident[EI_CLASS]), header.e_ident[EI_CLASS]);
    println!("Data encoding (e_ident[EI_DATA]): {} ({:#04X})", value_meanings::get_ei_data_meaning(header.e_ident[EI_DATA]), header.e_ident[EI_DATA]);
    println!("ELF specification version (e_ident[EI_VERSION]): {} ({:#04X})", value_meanings::get_ei_version_meaning(header.e_ident[EI_VERSION]), header.e_ident[EI_VERSION]);
    println!("Target OS and ABI (e_ident[EI_OSABI]): {} ({:#04X})", value_meanings::get_ei_osabi_meaning(header.e_ident[EI_OSABI]), header.e_ident[EI_OSABI]);
    println!("ABI version: (e_ident[EI_ABIVERSRION]): {:#04X}", header.e_ident[EI_ABIVERSION]);
    println!("Start of padding (e_ident[EI_PAD]): {:#04X}", header.e_ident[EI_PAD]);
    println!("Object file type (e_type): {} ({:#06X})", value_meanings::get_e_type_meaning(header.e_type), header.e_type);
    println!("Required architecture (e_machine): {} ({:#06X})", value_meanings::get_e_machine_meaning(header.e_machine), header.e_machine);
}

fn print_file_header_32(header: &Elf32_Ehdr) {
    println!("Magic: 0X7F ELF");
}
