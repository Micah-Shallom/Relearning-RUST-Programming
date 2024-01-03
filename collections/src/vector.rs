pub fn vectorfn() {
    // let mut v: Vec<i32> = vec![1,2,3];
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(1);
    v.push(2);

    //reading elements of vectors [2 ways]

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    //trying to (@chatgpt complete this commented line)
    let mut s = vec![1, 2, 3, 4, 5];
    let first = &s[0];

    println!("The first element is {first}");
    s.push(6);

    //iterating over vectors
    for i in &v {
        println!("{i}");
    }

    //mutablle iteration
    for i in &mut v {
        *i += 50;
    }

    //using an enum to sstore mutlitple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
