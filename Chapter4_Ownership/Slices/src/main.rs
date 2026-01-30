fn main() {
    let mut s = String::from("hello world!");

    let word = first_word_bad(&s);

    s.clear();  //empties the string, makes it equal to ""

    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];
    let punct = &s[11..];
    //println!("{}, {}{}",hello, world, punct);
}

fn first_word_bad(s: &String) -> usize {
    let bytes = s.as_bytes();   //converts string into an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        //i is the index count
        //item is the byte of the string
        if item == b' ' {   //if that "letter" is a space, we have our index
            return i;
        } 
    }

    s.len() //it's one word

    //this is bad because having to deal with indexes...
    //when you clear the original string is bad
}

fn first_word_good(s: &String) -> &str {
    //&str indicates string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]  //no spaces
}