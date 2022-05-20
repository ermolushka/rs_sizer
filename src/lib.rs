
mod filesystem;
use crate::filesystem::utils::{sort_metadata, print_vec, get_folders};
use std::path::Path;
use std::collections::HashMap;

pub fn get_info(path: &String, num_of_items: i32, level: &String, mut map: HashMap<String, f64>) {
    match level.as_str() {
        "folder" => {
            println!("folder");

            let mut count_vec: Vec<_> = map.iter().collect();
            let sorted_vec: Vec<(&String, &f64)> = sort_metadata(count_vec, num_of_items);
            print_vec(sorted_vec);

        },
        "file" => {
            println!("file");
            get_folders(Path::new(path), &mut map);

            let mut count_vec: Vec<_> = map.iter().collect();
            let sorted_vec: Vec<(&String, &f64)> = sort_metadata(count_vec, num_of_items);
            print_vec(sorted_vec);
        },
        _ => println!("unknown flag")
    }
}