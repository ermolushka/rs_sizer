use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, f64> = HashMap::new();
    get_entries(Path::new("/home/abc/Downloads"), &mut map);
    println!("{:?}", map);

}

fn get_entries(dir: &Path, map: &mut HashMap<String, f64>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            get_entries(path.as_path(), map);
        } else {
            let filename = path.to_str().unwrap().to_string();
            let file_size = fs::metadata(&filename)?.len();
            
            map.insert(filename, bytes_to_megabyte(file_size));
        
        }

    }
    Ok(())     
}

fn bytes_to_megabyte(bytes: u64) -> f64 {
    bytes as f64 * 0.000001 as f64
}
