#[allow(unused)]
fn print_type_of<T>(_: &T) {
    //https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
    println!("{}", std::any::type_name::<T>());
}


#[allow(unused)]
fn defining_and_creating_strings(){
    let mut s = String::new();

    let data = "initial contents";
    s = data.to_string();

    println!("Ex 1: {s}");

    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    println!("Ex 2: {s}");

    let s = String::from("initial contents");
    println!("Ex 3: {s}");

}

#[allow(unused)]
fn updating_a_string(){
    let mut s = String::from("foo");
    s.push_str("bar");  
    println!("Ex 1: {s}");  //foobar

    let mut s1 = String::from("foo");
    let s2 = "bar";
    //let s2 = String::from("bar"); // for this, push_str needs &s2.
    s1.push_str(s2);    
    //push_str does NOT take ownership away from s2
    println!("s2 is {s2}");

}

#[allow(unused)]
fn using_push(){
    let mut s = String::from("lo");
    s.push('l');
    //single character only
    println!("s = {s}");
}

#[allow(unused)]
fn concatenating(){
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    //s1 is no longer valid after using the + operator


    println!("{s3}\n");

    //can get unwieldy with multiple strings

    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");

    let s = format!("{s1}-{s2}-{s3}");  //does NOT take ownership
    //this is a STRING

    println!("s = {s}");
    //print_type_of(&s);

}


//Rust does not support indexing characters from strings
//i.e: let temp_char: char = s[0];

#[allow(unused)]
fn how_slicing_works(){
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    //you are not slicing the first 4 characters...
    //... you are slicing the first 4 BYTES
    println!("s is actually = {s}");    //3д, not Здра

    println!();
    for c in s.chars(){
        println!("{c}");
    }
    
    println!();
    for b in s.bytes(){
        println!("{b}");
    }
}

fn main() {
    //defining_and_creating_strings();
    //updating_a_string();
    //using_push();
    //concatenating();
    how_slicing_works();

}
