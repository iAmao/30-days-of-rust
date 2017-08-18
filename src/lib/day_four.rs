use std::io;

pub fn conditions () {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Error reading input");

    let input :i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    if input < 1 || input % 2 != 0 {
        println!("Weird");
    } else if input % 2 == 0 && (input >= 2 && input <= 5) {
        println!("Not Weird");
    } else if input % 2 == 0 && (input >= 6 && input <= 20) {
        println!("Weird");
    } else if input % 2 == 0 && input > 20 {
        println!("Not Weird");
    }
}
