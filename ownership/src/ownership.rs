

pub fn ownershipfn() {
    //integers implement the copy
    let x = 5;
    let y = x;

    println!("{x} {y}");

    //strings implements the move due to its potential ofbeing large....as such copying large values of strings in multiple places causes a problem
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s2);

    //also the same concept is applicable to the transfer of ownership in a function

    let s = String::from("hello");
    let x = 5;

    take_ownership(s);
    make_copy(x);

    println!("{x}"); //no error here cus integers copy and not move
    // println!("{}", s); //error here cus we try to access a value that was moved
}
fn make_copy(x: i32) {
    println!("{}", x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
