use std::io;

fn read_user_continue() -> bool {
    println!(
        "\nDeseja realizar outra operação?
(1) - Sim
(2) - Não"
);

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Falha na leitura!");
    let choice: u8 = choice.trim().parse().unwrap();

    if choice == 1 {
        true
    } else {
        false
    }
}

fn read_operation() -> u8 {
    let mut choice = String::new();
    println!(
        "Qual operação deseja realizar:
(1) - Celsius -> Fahrenheit
(2) - Fahrenheit -> Celsius
(3) - Celsius -> Kelvin
(4) - Kelvin -> Celsius
(5) - Fahrenheit -> Kelvin
(6) - Kelvin -> Fahrenheit"
    );

    io::stdin()
        .read_line(&mut choice)
        .expect("Falha na leitura!");
    let choice: u8 = choice.trim().parse().unwrap();

    choice
}

fn read_temperature() -> f64 {
    let mut temperature = String::new();
    println!("\nInsira o valor da temperatura: ");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Falha na leitura!");
    let temperature: f64 = temperature.trim().parse().unwrap();

    temperature
}

fn celsius_to_fahrenheit(temperature: f64) {
    let result: f64 = temperature * 1.8 + 32.;
    println!("\n{}°C = {}°F", temperature, result);
}

fn fahrenheit_to_celsius(temperature: f64) {
    let result: f64 = (temperature - 32.) / 1.8;
    println!("\n{}°F = {}°C", temperature, result);
}

fn celsius_to_kelvin(temperature: f64) {
    let result: f64 = temperature + 273.15;
    println!("\n{}°C = {}°K", temperature, result);
}

fn kelvin_to_celsius(temperature: f64) {
    let result: f64 = temperature - 273.15;
    println!("\n{}°K = {}°C", temperature, result);
}

fn fahrenheit_to_kelvin(temperature: f64) {
    let result: f64 = (temperature + 459.67) * (5. / 9.);
    println!("\n{}°F = {}°K", temperature, result);
}

fn kelvin_to_fahrenheit(temperature: f64) {
    let result: f64 = (temperature - 273.15) * 1.8 + 32.;
    println!("\n{}°K = {}°F", temperature, result);
}
fn main() {
    println!("--- Conversor de temperaturas ---");

    loop {
        match read_operation() {
            1 => celsius_to_fahrenheit(read_temperature()),
            2 => fahrenheit_to_celsius(read_temperature()),
            3 => celsius_to_kelvin(read_temperature()),
            4 => kelvin_to_celsius(read_temperature()),
            5 => fahrenheit_to_kelvin(read_temperature()),
            6 => kelvin_to_fahrenheit(read_temperature()),
            _ => println!("Escolha inválida!"),
        };

        if !read_user_continue() {
            println!("Obrigado por utilizar!");
            break;
        }
    }
}


