use std::{collections::HashMap, hash::Hash};
fn main() {
    let mut map = HashMap::new();

    map.insert(560086, "Bangalore-1".to_string());
    map.insert(560096, "Bangalore-2".to_string());
    map.insert(560034, "Bangalore-3".to_string());
    map.insert(560054, "Bangalore-4".to_string());
    map.insert(560021, "Bangalore-5".to_string());

    for (k, v) in &mut map {
        if *k == 560086{
           v.clear();
           v.push_str("Bengalore-1");
        }
        println!("key: {} value:{}", k, v);
    } // iterate the map and get keys and values

    let v = HashMap::get(&map, &560086);
    match v {
        Some(s) => println!("{}", s),
        None => println!("nothing in the map"),
    }
    // map.remove(&560086);
    let vec: Vec<i32> = Vec::with_capacity(100); // creates the space at once

    // add values to the vec random values --> crate.io 
    // create a map 
    // give me duplicate elemernts and nubbers in a vec .. show it in a map

    // create a vector with rand elemernts
    // delete duplicate elements from the vector
}

// key type must implement eq and hash traits
// the for in loops gives a tuple which is key and value


// take a vector
// 
