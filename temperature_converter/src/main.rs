use std::io;

fn main() {
    println!("Choose the input unit: [C/F]");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");
    let unit: char = unit.trim().parse().expect("Expected [C/F]");

    // @TODO: Update this later after learning array functions?
    if unit != 'C' && unit != 'F' && unit != 'c' && unit != 'f' {
        println!("Invalid Unit");
        return;
    }

    // @TODO: Change to const
    let unit_name = ["Celcius", "Fahrenheit"];
    let unit_index = if unit == 'C' || unit == 'c' { 0 } else { 1 };

    println!("Enter a temperature");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    let temperature: f32 = temperature.trim().parse().expect("Expected a number");

    let result_temperature: f32 = if unit_name[unit_index] == "Celcius" {
        (temperature * 9.0 / 5.0) + 32.0
    } else {
        (temperature - 32.0) * 5.0 / 9.0
    };

    println!(
        "{temperature} {} = {result_temperature} {}",
        unit_name[unit_index],
        unit_name[1 - unit_index]
    );
}
