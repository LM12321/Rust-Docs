#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}


fn example_one(){
    let scale = 2;  

    let rect1 = Rectangle {
        width: dbg!(30 * scale),    //can do this because dbg! returns ownership
        height: 50,
    };

    //println!("rect1 is {rect1}");     <-- will raise an error
    println!("rect1 is {rect1:?}"); // will only work with #[derive(Debug)]
    //adding # to that (EX: {rect1:#?}) will add new lines
    //also we can call dbg!

    dbg!(&rect1); //will take ownership so need to use a reference when calling it
    println!("The area of the rectangle is {} square pixels!", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}

fn main() {
    example_one();
}
