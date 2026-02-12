use std::collections::HashMap;  //to get Hashmap
use std::{io, vec};

//
fn add_person_to_department(database: &mut HashMap<String, Vec<String>>){

    println!("What's the department?"); //ask user for department
    let mut department: String =  String::new();    //create a department string

    //read line from user
    io::stdin() 
        .read_line(&mut department)
        .expect("Invalid input given");

    department = department.trim().to_string(); //remove newline
    //println!("department = {department}");  //show me   

    //do the same for the name
    let mut name: String = String::new();
    println!("What's the name?");
    io::stdin()
        .read_line(&mut name)
        .expect("Invalid input given");

    name = name.trim().to_string();
    //println!("name = {name}");
    
    //create/get a departnment vector for the hash
    let department_vector = database.entry(department).or_insert(Vec::new());

    department_vector.push(name);   //push the name to that vector

    sort_name_vector(department_vector);    //sort the vector
}

/*
match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
*/
fn sort_name_vector(vec: &mut Vec<String>){
    let n = vec.len();

    if n <= 1{
        return;
    }

    for i in 1..n{
        let key = vec[i].clone();

        let mut j = i - 1;
        let mut overflow = false;

        while (overflow == false) && (vec[j] > key){
            vec[j+1] = vec[j].clone();

            let overflow_check = j.checked_sub(1);
            match overflow_check{
                Some(_num) => {j -= 1; vec[j+1] = key.clone()},
                None => {overflow = true; vec[0] = key.clone()},
            }
        }
    }

    
}


fn get_menu_input() -> i32{
    loop {

        print_menu();

        let mut input = String::new();
        
        
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input given");


        let input: i32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!"); 
                continue;
            },
        };

        

        println!("Your input was: {input}");
        return input;
    }
}

fn print_out_department_from_user(database: &mut HashMap<String, Vec<String>>){

    println!("What's the department?"); //ask user for department
    let mut department: String =  String::new();    //create a department string

    //read line from user
    io::stdin() 
        .read_line(&mut department)
        .expect("Invalid input given");

    department = department.trim().to_string(); //remove newline
    //println!("department = {department}");  //show me   

    if let Some(vector_of_names) = database.get(&department){
        print_out_names_from_department(&department, vector_of_names)
    }else{
        println!("The department, {}, does not exist at the company!", department);
    }


}

fn print_out_names_from_department(department_name: &String, vector_of_names: &Vec<String>){
    println!("Here is everyone working in the {} departnment currently:", department_name);
    for name in vector_of_names{
        println!("\t{name}");
    }
}

fn print_out_company(database: &mut HashMap<String, Vec<String>>){
    for (department, vector_of_names) in database.iter(){
        print_out_names_from_department(department, vector_of_names);
    }
}

fn menu(database: &mut HashMap<String, Vec<String>>){

    loop {
        let input: i32 = get_menu_input();


        match input{
            0 => {
                println!("Goodbye!");
                break;
            },
            1 =>{
                add_person_to_department(database);
            },
            2 =>{
                print_out_department_from_user(database);
            },
            3 => {
                print_out_company(database);
            },
            _ =>{
                println!("Please input a valid number!");
            },
        }
    }
}

fn print_menu(){
    println!("Menu:");
    println!("\t0: Exit Program");
    println!("\t1: Add person to department");
    println!("\t2: Print out department");
    println!("\t3: Print out company");
}

fn main() {
    let mut database: HashMap<String, Vec<String>> = HashMap::new();  //initialize empty hashmap 
    database.insert(String::from("Math"), vec![String::from("Jamie")]);
    
    
    menu(&mut database);

}
