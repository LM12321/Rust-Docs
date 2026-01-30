

fn coin_example(){
    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) ->u8{
        match coin{
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
    

    let test_coin = Coin::Dime;
    println!("The value of test_coin is: {}.", value_in_cents(test_coin));
}


fn bind_to_values(){
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        //more...
    }
    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin{
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
            
        }
    }

    let test_coin = Coin::Quarter(UsState::Alabama);
    println!("The value of test_coin is: {}.", value_in_cents(test_coin));

    
}

fn option_match_pattern(){
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
        //if we forgot the None case, the compiler would yell at us
    }

    let five = Some(5);
    let six = plus_one(five);
    let var_none = plus_one(None);
    //println!("Five = {}\nSix = {}\nNone = {}",five, six, none);
}

fn catch_all_patters_and_underscore(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        5 => reroll(),
        _ => (), //this means we won't do anything if nothing it matched
    }
    // underscore can be 'other' as well
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}


fn main() {
    catch_all_patters_and_underscore()
}
