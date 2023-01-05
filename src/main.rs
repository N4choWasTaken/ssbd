use std::io;

fn main () {
    println!("Enter your time(hh:mm): ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let time: f32 = input.trim().parse().unwrap();

    let time_decimal = calculate_time_in_decimal(time);
    println!("Time in Decimal: {:.1}h", time_decimal);
}

fn calculate_time_in_decimal(time_decimal: f32) -> f32 {
    (time_decimal.floor() * 60.0 + (time_decimal - time_decimal.floor()) * 100.0) / 60.0
}
