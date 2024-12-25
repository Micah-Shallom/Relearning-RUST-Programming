// use std::io;


fn main() {
//    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//    let mut x  = 5;
//    println!("The vallue of x is: {}", x);

//    x = 6;   
//    println!("The value of x is: {}", x);

    // let x = 5;
    // let x = x + 1;

    // {
    //     let x =  x * 2;
    //     println!("The value of x in the inner scope is: {}", x);
    // }

    // println!("The value of x is : {x}");

    // let sum = 5 + 10;
    // println!("The sum of the operation is {sum}");

    // let _t: bool = true;

    // let tup = (500, 6.4, 1);
    // let (_x, y, _z) = tup;
    // println!("The value of y is: {y}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // //arrays
    // let a = [1,2,3,4,5];
    // let a: [i32; 5] = [1,2,3,4,5];
    // let a = [3;5]; //same as [3,3,3,3,3]

    //practical example
    // let a = [1,2,3,4,5];
    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("index entered is not a number");

    // let element = a[index];
    // println!("The value of the element at index {index} is: {element}");

    //understanding statments and expression
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("{y}")

    // let x = five();
    // println!("The value of x is: {x}");

    // let x = plus_one(x);
    // println!("This is the value of {x}")

}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1 //without the semi-colon it is an expression and it returns a value
    // x + 1; //with the semi-colon it is a statmenet and will throw an error
}