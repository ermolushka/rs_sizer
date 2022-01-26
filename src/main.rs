use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::HashMap;
use std::env;

fn main() {
    let mut map: HashMap<String, f64> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let path  = &args[1]; // "/home/abc/Downloads/test_images"
    get_entries(Path::new(path), &mut map);
    let mut count_vec: Vec<_> = map.iter().collect();
    let sorted_vec: Vec<(&String, &f64)> = sort_metadata(count_vec);
    println!("{:?}", sorted_vec);
    print_vec(sorted_vec);

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

fn sort_metadata<'a>(mut vec: Vec<(&'a String, &'a f64)>) -> Vec<(&'a String, &'a f64)> {
    vec.sort_by(|(k1, _), (k2, _)| k2.cmp(k1));
    vec.reverse();
    vec

}

fn print_vec<'a>(mut vec: Vec<(&'a String, &'a f64)>) -> () {
    let mut counter: i32 = 1;
    for entry in &vec {
        println!("line {}, file: {}, size (mb): {}", counter, entry.0, entry.1);
        counter += 1;
    }
}
