fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Please pass a path to a file.");
        return;
    }
    
    let path: std::path::PathBuf = std::path::PathBuf::from(&args[1]);
    let file_bytes: Vec<u8> = std::fs::read(path).expect("Failed to read file.");
    
    for i in 0..5 {
        println!("Byte {}: {:#02X}", i, file_bytes[i]);
    }

    return;
}
