use std::collections::HashMap;

#[allow(unused)]
fn hash_intro(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //score will be returned in an Option<&V>
    //unwrap_or enables the program to default to 0 if "team_name" is not found

    println!("{team_name}'s current score is: {score}\n");

    scores.insert(String::from("Blue"), 25);    //change Blue's score to 25

    for (key, value) in &scores {    //iterate through each key and value
        println!("{key}: {value}");
    }

    //iterating over a hashmap happens in an arbitrary order
}


#[allow(unused)]
fn managing_ownership(){
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    //println!("My favorite color is: {field_value}!");    //will raise error
    //hashmap now owns the fieldname and value...
    //...UNLESS references are used
}


#[allow(unused)]
fn adding_keys_if_key_isnt_present(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert((50));   
    //will return a mutable reference if exists
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
    //blue will remain 10
    //yellow will be added and be 50
}


#[allow(unused)]
fn updating_value_on_old_value(){
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

#[allow(unused)]
fn main() {
    //hash_intro();
    //managing_ownership();
    //adding_keys_if_key_isnt_present();
    updating_value_on_old_value();
}
