mod value_meanings;
use elf::abi::*;
use elf::file::*;
use libc::Elf64_Phdr;
use libc::Elf32_Phdr;

pub fn check_magic(bytes: &Vec<u8>) -> bool {
    return bytes.len() >= 4 && bytes[0] == 0x7F && bytes[1] == b'E' && bytes[2] == b'L' && bytes[3] == b'F';
}

pub fn print_elf_header(bytes: &Vec<u8>) {
    if bytes.len() >= size_of::<Elf64_Ehdr>() && bytes[EI_CLASS] == ELFCLASS64 {
        let elf_header: Elf64_Ehdr = unsafe { std::ptr::read(bytes.as_ptr() as *const Elf64_Ehdr) };
        print_elf_header_64(&elf_header);
    } else if bytes.len() >= size_of::<Elf32_Ehdr>() && bytes[EI_CLASS] == ELFCLASS32 {
        let elf_header: Elf32_Ehdr = unsafe { std::ptr::read(bytes.as_ptr() as *const Elf32_Ehdr) };
        print_elf_header_32(&elf_header);
    } else {
        println!("File has unknown architecture or bytes buffer is too small.");
    }
}

pub fn print_program_header(bytes: &Vec<u8>, index: u16) {
    if bytes.len() >= size_of::<Elf64_Ehdr>() && bytes[EI_CLASS] == ELFCLASS64 {
        let elf_header: Elf64_Ehdr = unsafe { std::ptr::read(bytes.as_ptr() as *const Elf64_Ehdr) };

        if elf_header.e_phoff == 0 {
            println!("File has no program header table.");
            return;
        }

        if bytes.len() < (elf_header.e_phnum * elf_header.e_phentsize) as usize {
            println!("Not enough bytes in buffer.");
            return;
        }

        if index == u16::MAX {
            // print all
            let mut  i: u16 = 0;
            while i < elf_header.e_phnum {
                let p_offset: isize = (elf_header.e_phoff + (i * elf_header.e_phentsize) as u64).try_into().unwrap();
                let program_header: Elf64_Phdr = unsafe { std::ptr::read((bytes.as_ptr().offset(p_offset)) as *const Elf64_Phdr) };
                print_program_header_64(&program_header);
                i = i + 1;
            }
        } else if index < elf_header.e_phnum {
                let p_offset: isize = (elf_header.e_phoff + (index * elf_header.e_phentsize) as u64).try_into().unwrap();
                let program_header: Elf64_Phdr = unsafe { std::ptr::read((bytes.as_ptr().offset(p_offset)) as *const Elf64_Phdr) };
                print_program_header_64(&program_header);
        } else {
            println!("Invalid index.");
            return;
        }
    } else if bytes.len() >= size_of::<Elf32_Ehdr>() && bytes[EI_CLASS] == ELFCLASS32 {
        // let elf_header: Elf32_Ehdr = unsafe { std::ptr::read(bytes.as_ptr() as *const Elf32_Ehdr) };
        // print_program_header_32(&header, index);
    } else {
        println!("File has unknown architecture or bytes buffer is too small.");
    }
}

fn print_elf_header_64(header: &Elf64_Ehdr) {
    println!();
    println!("ELF header 64-bit (Elf64_Ehdr)");
    println!("Magic (e_ident[0..4]): 0x7F ELF"); // assuming the header is valid
    println!("Architecture (e_ident[EI_CLASS]): {} ({:#04X})", value_meanings::get_ei_class_meaning(header.e_ident[EI_CLASS]), header.e_ident[EI_CLASS]);
    println!("Data encoding (e_ident[EI_DATA]): {} ({:#04X})", value_meanings::get_ei_data_meaning(header.e_ident[EI_DATA]), header.e_ident[EI_DATA]);
    println!("ELF specification version (e_ident[EI_VERSION]): {} ({:#04X})", value_meanings::get_ei_version_meaning(header.e_ident[EI_VERSION]), header.e_ident[EI_VERSION]);
    println!("Target OS and ABI (e_ident[EI_OSABI]): {} ({:#04X})", value_meanings::get_ei_osabi_meaning(header.e_ident[EI_OSABI]), header.e_ident[EI_OSABI]);
    println!("ABI version: (e_ident[EI_ABIVERSRION]): {:#04X}", header.e_ident[EI_ABIVERSION]);
    println!("Start of padding (e_ident[EI_PAD]): {:#04X}", header.e_ident[EI_PAD]);
    println!("Object file type (e_type): {} ({:#06X})", value_meanings::get_e_type_meaning(header.e_type), header.e_type);
    println!("Required architecture (e_machine): {} ({:#06X})", value_meanings::get_e_machine_meaning(header.e_machine), header.e_machine);
    println!("File version (e_version): {} ({:#04X})", value_meanings::get_ei_version_meaning(header.e_version as u8), header.e_version);
    println!("Entry point VA (e_entry): {:#04X}", header.e_entry);
    println!("Program header table file offset (e_phoff): {:#04X}", header.e_phoff);
    println!("Section header table file offset (e_shoff): {:#04X}", header.e_shoff);
    println!("Processor-specific flags (e_flags): {:#04X}", header.e_flags);
    println!("ELF header size (e_ehsize): {:#04X}", header.e_ehsize);
    println!("Size of a program header entry (e_phentsize): {:#04X}", header.e_phentsize);
    println!("Number of program header entries (e_phnum): {:#04X}", header.e_phnum);
    println!("Size of a section header entry (e_shentsize): {:#04X}", header.e_shentsize);
    println!("Number of section header entries (e_shnum): {:#04X}", header.e_shnum);
    println!("Section header table index of section name string table (e_shstrndx): {:#04X}", header.e_shstrndx);
    println!();
}

fn print_elf_header_32(header: &Elf32_Ehdr) {
    println!();
    println!("ELF header 32-bit (Elf32_Ehdr)");
    println!("Magic (e_ident[0..4]): 0x7F ELF"); // assuming the header is valid
    println!("Architecture (e_ident[EI_CLASS]): {} ({:#04X})", value_meanings::get_ei_class_meaning(header.e_ident[EI_CLASS]), header.e_ident[EI_CLASS]);
    println!("Data encoding (e_ident[EI_DATA]): {} ({:#04X})", value_meanings::get_ei_data_meaning(header.e_ident[EI_DATA]), header.e_ident[EI_DATA]);
    println!("ELF specification version (e_ident[EI_VERSION]): {} ({:#04X})", value_meanings::get_ei_version_meaning(header.e_ident[EI_VERSION]), header.e_ident[EI_VERSION]);
    println!("Target OS and ABI (e_ident[EI_OSABI]): {} ({:#04X})", value_meanings::get_ei_osabi_meaning(header.e_ident[EI_OSABI]), header.e_ident[EI_OSABI]);
    println!("ABI version: (e_ident[EI_ABIVERSRION]): {:#04X}", header.e_ident[EI_ABIVERSION]);
    println!("Start of padding (e_ident[EI_PAD]): {:#04X}", header.e_ident[EI_PAD]);
    println!("Object file type (e_type): {} ({:#06X})", value_meanings::get_e_type_meaning(header.e_type), header.e_type);
    println!("Required architecture (e_machine): {} ({:#06X})", value_meanings::get_e_machine_meaning(header.e_machine), header.e_machine);
    println!("File version (e_version): {} ({:#04X})", value_meanings::get_ei_version_meaning(header.e_version as u8), header.e_version);
    println!("Entry point VA (e_entry): {:#04X}", header.e_entry);
    println!("Program header table file offset (e_phoff): {:#04X}", header.e_phoff);
    println!("Section header table file offset (e_shoff): {:#04X}", header.e_shoff);
    println!("Processor-specific flags (e_flags): {:#04X}", header.e_flags);
    println!("ELF header size (e_ehsize): {:#04X}", header.e_ehsize);
    println!("Size of a program header entry (e_phentsize): {:#04X}", header.e_phentsize);
    println!("Number of program header entries (e_phnum): {:#04X}", header.e_phnum);
    println!("Size of a section header entry (e_shentsize): {:#04X}", header.e_shentsize);
    println!("Number of section header entries (e_shnum): {:#04X}", header.e_shnum);
    println!("Section header table index of section name string table (e_shstrndx): {:#04X}", header.e_shstrndx);
    println!();
}

fn print_program_header_64(header: &Elf64_Phdr) {
    println!();
    println!("Program header 64-bit (Elf64_Phdr)");
    println!("Offset: {:#X}", header.p_offset);
    println!();
}

fn print_program_header_32(header: &Elf32_Phdr) {
    println!();
    println!("Program header 32-bit (Elf32_Phdr)");
    println!("Offset: {:#X}", header.p_offset);
    println!();
}
