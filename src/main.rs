use std::collections::HashMap;
use std::env;
extern crate fs_extra;
use rs_sizer::get_info;

fn main() {
    let mut map: HashMap<String, f64> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let path  = &args[1];


    let num_of_items: i32 = match args[2].parse() {
        Ok(i) => {i},
        Err(_) => {
            println!("error: second argument not an integer");
            return;
        },
    };
    let level: &String = &args[3];
    get_info(path, num_of_items, level, map);

}
