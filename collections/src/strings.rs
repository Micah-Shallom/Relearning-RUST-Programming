pub fn stringfn() {
    let data = "initial contents"; //string literal
    let s = data.to_string();

    let s = String::from("initial contents");

    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("नम�ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let _hello = String::from("你好");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //concatenation with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //note that s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
}
