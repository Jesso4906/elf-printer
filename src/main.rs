mod elf_viewer;

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
   
    if !elf_viewer::check_magic(&file_bytes) {
        println!("Invalid ELF binary.");
        return;
    }

    if args.len() == 2 || args[1] == "-eh" || args[1] == "--elf-header" {
        elf_viewer::print_file_header(&file_bytes);
    }
    
    return;
}

fn print_help() {
    println!("You must pass a file path as the final argument.");
    println!("If you do not provide any other arugments, the ELF header will be printed.");
    println!("-h, --help: print this menu.");
    println!("-eh, --elf-header: display information from the file's ELF header.");
}
