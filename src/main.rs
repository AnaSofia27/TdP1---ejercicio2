use ejercicio_2::file::File;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Try: ./program file_name_1 file_name_2");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let file1 = File::new(filename1);
    let file2 = File::new(filename2);

    file1.diff(&file2);
}
