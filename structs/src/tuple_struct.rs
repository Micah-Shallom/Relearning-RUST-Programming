struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn tuplestructsfn() {
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

struct AlwaysEqual;

pub fn unitstructfn() {
    let _subject = AlwaysEqual;
}

pub fn example_program() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_refactored(rect1)
    );

    struct_operation();
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_refactored(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn struct_operation() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        structfn(&rect1)
    );
    dbg!(&rect1);
}

fn structfn(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
