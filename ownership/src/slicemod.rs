pub fn slicefn() {
    //slices do not have ownership but they are a type of references
    let s: String = String::from("This is the best time of life");

    let word = first_word(&s);
    println!("{word}");


    let t = String::from("hello world");

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}