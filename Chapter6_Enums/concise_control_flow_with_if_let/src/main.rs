fn if_let_first_example_bad(){
    let config_max = Some(3u8);
    match config_max{
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),    
    }
    //bad because we have to always include _ => () for the None case
    
}

fn if_let_first_example_good(){
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }else{
        //can also add an else statement to this for the _ case
    }
    //good because only one conditional
    
}



fn main() {
    if_let_first_example_bad();
    if_let_first_example_good();
}
