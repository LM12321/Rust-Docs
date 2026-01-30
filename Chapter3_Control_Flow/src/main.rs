fn main() {
    println!("Hello, world!");
    let result = fibonacci(59);
    println!("Fibonacci result: {}", result);
    let temp: f32 = 50 as f32;
    let ftc: bool = true;
    let temp_result = fahrenheit_celsius_converter(temp, ftc);
    println!("Temp result: {}", temp_result);

}

fn fibonacci(num: u64) -> u64{

    if num == 0{
        return 0;
    }else if num == 1{
        return 1;
    }

    let mut counter = 0;
    let mut a = 0;
    let mut b = 1;

    while counter < num - 1{
        let temp = a + b;
        
        a = b;
        b = temp;
        counter += 1; 
    }
    
    b
}


fn fahrenheit_celsius_converter(temp:f32, ftc:bool) -> f32{
    if ftc{
        let result:f32 = (temp - (32 as f32)) * (5/9) as f32;
        return result;
    }else{
        let result:f32 = (temp * (9/5) as f32) + 32 as f32;
        return result;
    }
    
}