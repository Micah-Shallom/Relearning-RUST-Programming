pub fn br_reference() {
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    //rust allow us to return multiple values using a tuple
    // let s1 = String::from("hello");
    // let(s2, len) = calculate_length(s1);
    // println!("The length of {} is {}.", s2, len);

    //using a pointer I can dispel this problem
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}.", s1, len);

    //when we try to modify something we are borrowing it throws an errpr
    //we have to mutably provide xonsent to allow for modification of the value
    let mut change = String::from("hello");
    changefn(&mut change);

    //anaother rule of rust realted to mutable references is that there can only be a single mutable reference to a value. creating multiple causes errors
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    //dangling references
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing)
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
//     //     Because s is created inside dangle , when the code of dangle is ﬁnished, s will be
//     // deallocated. But we tried to return a reference to it. That means this reference would be
//     // pointing to an invalid String . That’s no good! Rust won’t let us do this

//     //the solution here is to return s directly
// }

fn dangle() -> String {
    let s = String::from("hello");
    s
}

fn changefn(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn calculate_length(s: String) -> (String, usize) {
//     // let length = s.len();

//     // (s, length)
// }

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

