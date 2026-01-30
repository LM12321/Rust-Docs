
fn print_days_of_christmas(day:u8){
    if day == 1{
        println!("On the 1st day of Christmas my true love gave to me:");
    }else if day == 2{
        println!("On the 2nd day of Christmas my true love gave to me:");
    }else if day == 3{
        println!("On the 3rd day of Christmas my true love gave to me:");
    }else{
        println!("On the {}th day of Christmas my true love gave to me:", day);
    }

    if day == 12{
        println!("\tTwelve drummers drumming");
    }
    if day >= 11{
        println!("\tEleven pipers piping");
    }
    if day >= 10{
        println!("\tTen lords a-leaping");
    }
    if day >= 9{
        println!("\tNine ladies dancing");
    }
    if day >= 8{
        println!("\tEight maids a-milking");
    }
    if day >= 7{
        println!("\tSeven swans a-swimming");
    }
    if day >= 6{
        println!("\tSix geese a-laying");
    }
    if day >= 5{
        println!("\tFive golden rings");
    }
    if day >= 4{
        println!("\tFour calling birds");
    }
    if day >= 3{
        println!("\tThree French hens");
    }
    if day >= 2{
        println!("\tTwo turtle doves");
    }

    println!("\tAnd a partridge in a pear tree");


}



fn main() {
    for day in 1..13{
        print_days_of_christmas(day);
    }

}
