use std::io;

pub fn operators() {
    let mut meal_cost = String::new();
    let mut tip_percent = String::new();
    let mut tax_percent = String::new();


    // READ AND PARSE INPUT
    io::stdin().read_line(&mut meal_cost)
        .expect("Could not read input");

    io::stdin().read_line(&mut tip_percent)
        .expect("Could not read input");

    io::stdin().read_line(&mut tax_percent)
        .expect("Could not read input");

    let meal_cost :f32 = match meal_cost.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };

    let tip_percent :f32 = match tip_percent.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };

    let tax_percent :f32 = match tax_percent.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };

    // END READ PARSE

    let total_cost :i32 = (meal_cost + (meal_cost * (tip_percent/100.0)) + (meal_cost * (tax_percent/100.0))) as i32;

    println!("The total meal cost {} dollars", total_cost.to_string());
}
