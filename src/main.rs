use mila::fe9_arc;
use mila::LZ10CompressionFormat;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Incorrect number of arguments.");
        println!("Usage: fe9cmp <path to .cmp file to unpack>");
    }

    let path = Path::new(&args[1]);
    if path.is_file() {
        let file = std::fs::read(path).expect("ERROR: Failed to read input file.");
        let decompressed = LZ10CompressionFormat {}
            .decompress(&file)
            .expect("ERROR: Failed to LZ10 decompress input file.");
        let arc = fe9_arc::parse(&decompressed)
            .expect("ERROR: Failed to parse input file as an FE9 archive.");

        let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
        for (k, v) in arc {
            let mut write_path = std::env::current_dir()
                .expect("ERROR: Unable to determine the working directory.")
                .clone();
            write_path.push(&file_name);
            std::fs::create_dir_all(&write_path)
                .expect("ERROR: Failed to create output directories.");
            write_path.push(k);
            std::fs::write(write_path, v).expect("ERROR: Failed to write file.");
        }
    } else {
        println!("ERROR: Path is not a valid file.")
    }
}
