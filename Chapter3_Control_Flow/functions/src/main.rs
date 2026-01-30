fn main() {
    println!("Hello, world!");

    //another_function(5, 10);
    //print_labeled_measurement(5, 'h');
    scope_test();
}

fn another_function(x: i32, y:i32) {
    println!("Another function.");
    println!("The value of x and y is: {x} & {}", y);
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn scope_test(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}