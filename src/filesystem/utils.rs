
use std::path::Path;
use std::collections::HashMap;
use std::cmp;
use std::io;
use std::fs;
use fs_extra::dir::get_size;


pub fn get_files(dir: &Path, map: &mut HashMap<String, f64>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            get_files(path.as_path(), map);
        } else {
            let filename = path.to_str().unwrap().to_string();
            let file_size = fs::metadata(&filename)?.len();
            
            map.insert(filename, bytes_to_megabyte(file_size));
        
        }

    }
    Ok(())     
}


pub fn get_folders(dir: &Path, map: &mut HashMap<String, f64>) -> io::Result<()> {
    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| {
            let file_size = get_size(&e.path()).unwrap();
            map.insert(e.path().to_str().unwrap().to_string(), bytes_to_megabyte(file_size))
        }))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(())
    
}


pub fn bytes_to_megabyte(bytes: u64) -> f64 {
    bytes as f64 * 0.000001 as f64
}

pub fn sort_metadata<'a>(mut vec: Vec<(&'a String, &'a f64)>, limit: i32) -> Vec<(&'a String, &'a f64)> {
    vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    vec.reverse();
    let max_size = cmp::min(vec.len(), limit as usize);
    vec[..max_size].to_vec()

}

pub fn print_vec<'a>(mut vec: Vec<(&'a String, &'a f64)>) -> () {
    let mut counter: i32 = 1;
    for entry in &vec {
        println!("line {}, file: {}, size (mb): {}", counter, entry.0, entry.1);
        counter += 1;
    }
}