

fn pig_latinfy(string: &String) -> String{
    let vowels = String::from("aeiou");
    let mut result = String::new();

    for c in string.chars(){
        //println!("{c}");
        for vowel in vowels.chars(){
            if vowel == c{
                result = format!("{string}-hay");
                return result;
            }
        }
        break;
    }

    result.push_str(&string[1..]);
    result.push_str("-");
    result.push_str(&string[..1]);
    result.push_str("ay");
    result
}

fn main() {
    let test_string = String::from("ba");
    let result = pig_latinfy(&test_string);
    println!("{result}");
}
