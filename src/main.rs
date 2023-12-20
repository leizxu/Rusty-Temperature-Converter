use std::io;

fn main() {
    println!("Temperature Converter!");
    println!("Is your temperature unit in C or F?");

    let mut units: String = String::new();

    io::stdin()
        .read_line(&mut units)
        .expect("what?");

    let units = units.trim();

    println!("What is the {units} temperature?");
    let mut degree = String::new();
    io::stdin()
        .read_line(&mut degree)
        .expect("Failed to Read Line");

    let degree: f64 = degree
        .trim()
        .parse()
        .expect("Input entered was not a number");

    if units == "f" {
        let conv_degree: f64 = f2c(degree);
        println!("{degree}{units} is {conv_degree}° in celsius")
    } else if units == "c"{
        let conv_degree:f64 = c2f(degree);
        println!("{degree}{units} is {conv_degree} in F")
    }

    }

fn f2c(f: f64) -> f64 {
    let c = (f - 32.0) * (5.0/9.0);
    return c;
}

fn c2f(c: f64) -> f64 {
    let f = c * (9.0/5.0) + 32.0;
    return f;
}





