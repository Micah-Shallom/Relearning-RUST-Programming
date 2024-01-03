use std::{collections::HashMap, hash::Hash};

pub fn hashmapfn() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //adding key and value only if a key isnt present
    scores.entry(String::from("green")).or_insert(34);
    println!("{:?}", scores);

    //updating a value based on a old value

    let text = "This is Shallom Micah Bawa";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}