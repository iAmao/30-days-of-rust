use std::{io};

pub fn data_types() {
    let i: i32 = 4;
    let double: f32 = 4.0;
    let s = "You only live once";

    let mut u_int = String::new();
    let mut u_float = String::new();
    let mut u_string = String::new();

    println!("Input an integer >");

    io::stdin().read_line(&mut u_int)
        .expect("Failed to read your input");

    let u_int: i32 = match u_int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    if u_int < 1 {
        return println!("You cannot input alphabets or number less than 1");
    }

    println!("Input a float >");

    io::stdin().read_line(&mut u_float)
        .expect("Failed to read your input");

    let u_float: f32 = match u_float.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };

    if u_float < 1.0 {
        return println!("You cannot only input floats >= 1");
    }

    println!("Input a string >");

    io::stdin().read_line(&mut u_string)
        .expect("Failed to read your input");

    println!("{} + {}> {}", i, u_int,  (u_int + i).to_string());

    println!("{} + {}> {}", double, u_float, (u_float + double).to_string());

    println!("{} + {}> {} {}", s, u_string, s, u_string);
}
