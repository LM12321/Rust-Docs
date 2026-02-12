
#[allow(unused)]
fn ex_one(){
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];   //i32 is the default type

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let third: &i32 = &v1[2];
    println!("The third element is {third}");

    let forth: Option<&i32> = v2.get(3);    //will return None if out of range
    //let forth_error: &i32 = &v2[3]; //will panic as it is out of range

    match forth {
        Some(forth) => println!("The forth element is {forth}"),
        None => println!("There is no forth element."),
    }

}

#[allow(unused)]
fn ex_two(){
    //will not work as we can't have reference and then push
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];  //active reference on mutable vector

    //v.push(6);  //can't do this with reference
    //might change pointer value
    println!("The first element is {first}");
}

#[allow(unused)]
fn iterating(){
    let v = vec![100, 32, 57];
    for i in &v{
        println!("{i}");
    }
}

#[allow(unused)]
fn iterating_mut(){
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; //will add 50 to each thing
        println!("{i}");
    }

}

#[allow(unused)]
fn enum_multiple_types(){
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //this will work!!
    //will allow compiler to know at run time (with a match)...
    //...that every case is handled
    
}

fn main() {
    //ex_one();
    //ex_two();
    //iterating();
    //iterating_mut();
    enum_multiple_types();

    //a vector will drop its elements outside of
}
