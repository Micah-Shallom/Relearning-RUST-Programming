fn main() {
    //integers implement the copy 
    let x = 5;
    let y = x;

    println!("{x} {y}");

    //strings implements the move due to its potential ofbeing large....as such copying large values of strings in multiple places causes a problem
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s1);

}
