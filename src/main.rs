use std::io;

fn main() {
    println!("Hello, world!");
    /*

    ###########################
        fibonacci sequence
    ###########################
    println!("Enter number to get fic num of: ");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: u32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let f = fib(x);

    println!("Fib of {} is {}", x, f);
    */

    /*

    ###########################
        celsius to fahrenheit
    ###########################
    println!("Input Celsius degree:");
    let mut c = String :: new();
    io::stdin().read_line(&mut c).expect("Filed to read line");

    let c: f64 = match c.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    let f = c_to_f(c);
    println!("{} degrees Celsius is {} degrees Fahrenheit", c, f);
    */

    /*

    ###########################
        12 days of christmas
    ###########################
    twelve_days()
    */
    
}

fn fib(x:u32) -> u32{
    if x == 0 {
        0
    } else if x == 1 {
        1
    }else{
        fib(x-1) + fib(x-2)
    }
}

fn c_to_f(x:f64) -> f64{
    x * (9.0/5.0) + 32.0
}

fn twelve_days(){
    let days = [
    "first", "second", "third", "fourth", "fifth", "sixth", 
    "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    for (day_num, day) in days.iter().enumerate() {
        println!("For the {} day of Christmas my true love sent to me", day);

        for gift_day in (1..(day_num + 1)).rev(){
            if gift_day == 1 && day_num != 1 {
                print!("and ");
            }
            match gift_day {
                1 => println!("a Patridge in a Pear Tree"),
                2 => println!("Two Turtle Doves"),
                3 => println!("Three French Hens"),
                4 => println!("Four Calling Birds"),
                5 => println!("Five Golden Rings"),
                6 => println!("Six Geese a Laying"),
                7 => println!("Seven Swans a Swimming"),
                8 => println!("Eight Maids a Milking"),
                9 => println!("Nine Ladies Dancing"),
                10 => println!("Ten Lords a Leaping"),
                11 => println!("Eleven Pipers Piping"),
                12 => println!("12 Drummers Drumming"),
                _ => {}
            }
        }
        
        println!("\n");
    }
}