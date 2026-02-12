//use math::round;
use std::collections::HashMap;

fn insert_sort(list_of_ints:  &mut Vec<i32>){
    
    if list_of_ints.len() <= 1{
        return
    }

    let n = list_of_ints.len();
    //println!("n = {n}");

    for i in 1..n{
        //println!("i = {i}");
        let key: i32 = list_of_ints[i];

        let mut j = i - 1;
        let mut overflow = false;

        while overflow == false && list_of_ints[j] > key{
            //println!("\tj = {j}");
            list_of_ints[j+1] = list_of_ints[j];

            let overflow_check = j.checked_sub(1);
            match overflow_check{
                Some(i) => {j -= 1; list_of_ints[j+1] = key;},
                None => {overflow = true; list_of_ints[0] = key},
            }
            
            //print_out_vector(list_of_ints);
        }


    }
}

fn print_out_vector(list_of_ints: &Vec<i32>){
    println!("{list_of_ints:?}");
}

fn find_median(list_of_ints: &mut Vec<i32>) -> i32 {
    insert_sort(list_of_ints);

    if list_of_ints.len() % 2 == 0{  //if even
        let midpoint_two = list_of_ints.len() / 2;
        let midpoint_one = midpoint_two - 1;

        return (list_of_ints[midpoint_one] + list_of_ints[midpoint_two])/2; //return median of both
    }else {     //if odd
        let midpoint = list_of_ints.len() / 2;  //will return the floor
        return list_of_ints[midpoint];  //return midpoint
    }   
}

fn find_max(map_of_ints: &HashMap<i32, i32>) -> i32{
    let mut max_count: i32 = 0;
    let mut current_max: i32 = 0;
    for (key, value) in map_of_ints{
        //println!("{key}: {value}");
        if (*value > max_count){
            current_max = *key;
            max_count = *value;
        } 
    }

    current_max
}

fn find_mode(list_of_ints: &Vec<i32>) -> i32{
    let mut map: HashMap<i32, i32> = HashMap::new();

    for int in list_of_ints{
        let count = map.entry(*int).or_insert(0);
        *count += 1;
    }

    find_max(&map)


}

fn main() {
    let mut list_of_ints: Vec<i32> = vec![40, 30, 20, 20, 20, 69, 20348];
    print_out_vector(&list_of_ints);
    let median = find_median(&mut list_of_ints);
    print_out_vector(&list_of_ints);

    println!("Median is {median}");
    let mode = find_mode(&list_of_ints);
    println!("Mode is {mode}");


}
