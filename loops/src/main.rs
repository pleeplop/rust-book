use std::io;

fn main() {
    the_twelves_days_of_christmas_song();
}

fn the_twelves_days_of_christmas_song() {
    const GIFTES: [(&str, &str); 12] = [
        ("first","A partridge in a pear tree"),
        ("second","Two turtle doves"),
        ("third","French hens"),
        ("fourth","Four calling birds"),
        ("fifth","Five gold rings"),
        ("sixth","Six geese a-laying"),
        ("seventh","Seven swans a-swimming"),
        ("eighth","Eigth maids a-milking"),
        ("ninth","Nine ladies dancing"),
        ("tenth","Ten lords a-leaping"),
        ("eleventh","Eleven pipers piping"),
        ("twelfth","Twelve drummers drumming"),
        ];
    for day in {0..12} {
        println!("day is: {}", day);
        println!("On the {} day of Christmas my true love sent to me", GIFTES[day].0);
        for gift in {0..day+1}.rev() {
            println!("{}", GIFTES[gift].1);
        }
        println!("");
    }
    
}

fn fibonacci() {
    println!("Please input a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    let number = input.trim().parse()
        .expect("Failed to read a number");

    let mut result = 1;
    let mut n2 = 1;
    let mut n1 = 1;
    for _n in {2..number} {
        result = n2 + n1;
        n1 = result;
        n2 = n1;
    }
    println!("The nth Fibonacci number is: {}", result);
}

fn temperature_converter() {
    println!("Please input a number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    let degree = input.trim().parse()
        .expect("Failed to read a number");
    let celcius = farenheit_to_celcius(degree);
    let farenheit = celcius_to_farenheit(degree);
    println!("celcius: {} farenheit: {}", celcius, farenheit);
}

fn farenheit_to_celcius(farenheit: f64) -> f64 {
(5.0 / 9.0) * (farenheit - 32.0)
}

fn celcius_to_farenheit(celcius: f64) -> f64 {
    ((celcius * 9.0) / 5.0) + 32.0
}