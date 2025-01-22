use std::fmt::Error;

fn main() {
    let src = ".";
    read_dir(src).unwrap();
}

fn read_dir(src: &str) -> Result<(), Error> {
    let dir = std::fs::read_dir(src).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        
        if is_file(entry.path().to_str().unwrap()) {
            println!("{}", entry.path().file_name().unwrap().to_string_lossy());
        } else {
            println!("{}/", entry.path().to_string_lossy());
        }

    }
    Ok(())

}

fn is_file(src: &str) -> bool {
    let metadata = std::fs::metadata(src).unwrap();
    metadata.is_file()
}