use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

fn main() {
    let mut vec1: Vec<&str> = Vec::new();
    get_entries(Path::new("/home/abc/Documents/rs_sizer"));

}

fn get_entries(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            get_entries(path.as_path());
        } else {
            let mut filename = path.to_str().unwrap();
            //vec1.push(&filename);
            println!("{:?}", path);
        }

    }
    Ok(())     
}
