mod elf_printer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Please pass a path to a file. Use -h or --help for help.");
        return;
    }

    if args[1] == "-h" || args[1] == "--help" {
        print_help();
        return;
    }
    
    let path: std::path::PathBuf = std::path::PathBuf::from(&args[args.len() - 1]);
    let file_bytes: Vec<u8> = std::fs::read(path).expect("Failed to read file.");
   
    if !elf_printer::check_magic(&file_bytes) {
        println!("Invalid ELF binary.");
        return;
    }

    if args.len() == 2 || args[1] == "-eh" || args[1] == "--elf-header" {
        elf_printer::print_elf_header(&file_bytes);
    } else if args[1] == "-ph" || args[1] == "--program-header" {
        let index: u16 = match args[2].parse() {
            Ok(n) => { n },
            Err(_) => { u16::MAX }
        };

        elf_printer::print_program_header(&file_bytes, index);
    } else if args[1] == "-sh" || args[1] == "--section-header" {
        let index: u16 = match args[2].parse() {
            Ok(n) => { n },
            Err(_) => { u16::MAX }
        };

        elf_printer::print_section_header(&file_bytes, index);
    } else if args[1] == "-s" || args[1] == "--dump-symbols" {
        elf_printer::print_symbol(&file_bytes);
    }

    
    return;
}

fn print_help() {
    println!("Elf Analyzer is a program that can be used to view information about ELF binaries.");
    println!();
    println!("Usage: elfa [OPTIONS] [FILE PATH]");
    println!();
    println!("You must pass a file path as the final argument.");
    println!("If you do not provide any other arugments, the ELF header will be printed.");
    println!();
    println!("Options:");
    println!("-h, --help: print this menu.");
    println!("-eh, --elf-header: display information from the file's ELF header.");
    println!("-ph, --program-header [INDEX]: display information about a program header by index. If no index is provided, all program headers will be printed.");
    println!("-sh, --section-header [INDEX]: display information about a section header by index. If no index is provided, all section headers will be printed.");
    println!("-s, --dump-symbols: dump all symbols found in the .symtab section.");
}
