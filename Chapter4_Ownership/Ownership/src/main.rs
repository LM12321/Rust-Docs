fn main(){
    //scope_example();    //scope example

    let s = String::from("Hello!");
    does_not_take_ownership(&s);
    takes_ownership(s);

    //println!("{s}");   
    //throws an error as s is already dropped
    //String can't do .clone()

}

fn scope_example() {
    
    let s = "hello!";    //s outer scope
    {
        let s = "hello again!"; //only in *this* scope

        println!("{}", s); //hello again!
    }
    println!("{}", s);    //"hello"
    //a is no longer in that scope
}   



fn takes_ownership(rand_str: String){
    println!("{rand_str}");
}

fn does_not_take_ownership(rand_str: &String){
    println!("{rand_str}");     //use reference, scope isn't lost
}

fn reference_rules(){
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
}